use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResp<'a> {
    message: &'a str,
}

pub async fn resp_not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResp {
        message: "Page not found",
    })
}
