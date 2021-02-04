use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::client::WsClientError::SendRequest;
use actix_web::middleware::errhandlers::ErrorHandlerResponse::Response;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpResponse, Responder};
use color_eyre::SectionExt;
use futures::future::{ok, Ready};
use futures::Future;
use serde::{Deserialize, Serialize};

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
            let path = req.uri().path();
            let mut left = "Nan".to_string();
            let mut limit = "Nan".to_string();
            println!("{}", path);
            if path == "/" || path == "/metrics" {
                return Ok(svc.call(req).await.unwrap());
            }
            let header = req.headers().get("Authorization");
            let is_valid: (i32, String, String) = (|| async {
                let header = match header {
                    Some(header) => header,
                    None => return (403, limit, left),
                };
                let header = match header.to_str() {
                    Ok(header) => header,

                    Err(_) => return (500, limit, left),
                };
                //let client = req.app_data::<reqwest::Client>().expect("NO CLIENT");
                //let _dat = req.app_data::<web::Data<MonVec>>().expect("FAKE BALLS");
                //let client = reqwest::Client::new();
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
                    (true, true) => return (429, limit, left),
                    (true, false) => return (200, limit, left),
                    (false, _) => return (403, limit, left),
                }
            })()
            .await;
            match is_valid.0 {
                200 => {
                    let mut r: ServiceResponse<B> = svc.call(req).await.unwrap();
                    let h = r.headers_mut();
                    h.insert("a".parse().unwrap(), is_valid.1.parse().unwrap());
                    h.insert("b".parse().unwrap(), is_valid.2.parse().unwrap());
                    return Ok(r);
                }
                403 => {
                    return Ok(req.into_response(
                        HttpResponse::Forbidden()
                            .json(ErrorResp {
                                message: "Unauthorized",
                            })
                            .into_body(),
                    ))
                }
                429 => {
                    return Ok(req.into_response(
                        HttpResponse::TooManyRequests()
                            .header("X-Ratelimit-Limit", is_valid.1)
                            .header("X-Ratelimit-Left", is_valid.2)
                            .json(ErrorResp {
                                message: "Ratelimited",
                            })
                            .into_body(),
                    ));
                }
                _ => {
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(ErrorResp {
                                message: "Server having issues",
                            })
                            .into_body(),
                    ));
                }
            };
        })
    }
}
