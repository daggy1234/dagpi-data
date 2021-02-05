use crate::error::ErrorResp;
use actix_web::HttpResponse;

pub async fn resp_not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResp {
        message: "Page not found",
    })
}
