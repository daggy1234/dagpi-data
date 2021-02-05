use crate::middlewares::Stat;
use crate::Request;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    auth: bool,
    ratelimited: bool,
    premium: bool,
    ratelimit: i32,
    left: i32,
}

#[derive(Serialize)]
struct ErrorResp<'a> {
    message: &'a str,
}

pub struct RequiresAuth;
//FULL CREDIT FOR THIS: https://git.travitia.xyz/Adrian/cdnup/-/blob/master/src/auth.rs

//THANK YOU TO ADRIAN FOR THIS AUTH CODE
// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body

impl<S: 'static, B> Transform<S> for RequiresAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequiresAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequiresAuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct RequiresAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}
#[allow(clippy::type_complexity)]
impl<S, B> Service for RequiresAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();
        Box::pin(async move {
            let mut left = "Nan".to_string();
            let mut limit = "Nan".to_string();

            let path = req.path();
            if path == "/" || path == "/metrics" {
                return Ok(svc.call(req).await.unwrap());
            }

            let is_valid: (i32, String, String) = (|| async {
                let header = req.headers();
                let auth_head = header.get("Authorization");
                let header = match auth_head {
                    Some(header) => header,
                    None => return (403, limit, left),
                };
                let header = match header.to_str() {
                    Ok(header) => header,

                    Err(_) => return (500, limit, left),
                };
                let client = req.app_data::<web::Data<reqwest::Client>>().unwrap();
                let auth_url = std::env::var("auth_url").expect("WE NEED A URL BRUH");
                let req_url: String = format!("{}/auth/{}", auth_url, header);
                let resp = client
                    .get(&req_url)
                    .header(
                        "Authorization",
                        &std::env::var("Token").expect("NO env token"),
                    )
                    .send()
                    .await;
                let resp = match resp {
                    Ok(resp) => resp,
                    Err(_) => return (500, limit, left),
                };
                let resp: AuthResponse = match resp.json().await {
                    Ok(resp) => resp,
                    Err(_) => return (500, limit, left),
                };
                limit = format!("{}", resp.ratelimit);
                left = format!("{}", resp.left);
                match (resp.auth, resp.ratelimited) {
                    (true, true) => (429, limit, left),
                    (true, false) => (200, limit, left),
                    (false, _) => (403, limit, left),
                }
            })()
            .await;
            match is_valid.0 {
                200 => {
                    let header = req.headers();
                    let ua = match header.get("User-Agent") {
                        Some(h) => h.to_str().unwrap().to_string(),
                        None => "No UserAgent".to_string(),
                    };
                    let token = header
                        .get("Authorization")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    let mut r: ServiceResponse<B> = svc.call(req).await.unwrap();
                    let h = r.headers_mut();
                    h.insert("a".parse().unwrap(), is_valid.1.parse().unwrap());
                    h.insert("b".parse().unwrap(), is_valid.2.parse().unwrap());
                    let freq = r.request().clone();
                    actix_web::rt::spawn(async move {
                        let tx = &freq.app_data::<web::Data<Sender<Request>>>().unwrap();
                        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
                        let cmd = Request::Stat {
                            payload: Stat {
                                user_agent: ua,
                                route: freq.path().to_string(),
                                api: "data".to_string(),
                                token,
                            },
                            resp: resp_tx,
                        };
                        tx.send(cmd).await.unwrap();
                        let res = resp_rx.await.unwrap();
                        println!("Recieved Status {:?}", res);
                    });
                    Ok(r)
                }
                403 => Ok(req.into_response(
                    HttpResponse::Forbidden()
                        .json(ErrorResp {
                            message: "Unauthorized",
                        })
                        .into_body(),
                )),
                429 => Ok(req.into_response(
                    HttpResponse::TooManyRequests()
                        .header("X-Ratelimit-Limit", is_valid.1)
                        .header("X-Ratelimit-Left", is_valid.2)
                        .json(ErrorResp {
                            message: "Ratelimited",
                        })
                        .into_body(),
                )),
                _ => Ok(req.into_response(
                    HttpResponse::InternalServerError()
                        .json(ErrorResp {
                            message: "Server having issues",
                        })
                        .into_body(),
                )),
            }
        })
    }
}
