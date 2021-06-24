use actix_web::{
    dev::{Body, ServiceResponse},
    http::StatusCode,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    HttpResponse, Result,
};
use serde_json::json;
pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new()
        .handler(StatusCode::METHOD_NOT_ALLOWED, method_not_allowed)
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
}

fn internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::InternalServerError()
                .json(json!({"message": "Internal Server Error"}))
                .into_body(),
        ),
    ))
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound()
                .json(json!({"message": "page not found"}))
                .into_body(),
        ),
    ))
}

fn method_not_allowed<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let resa = res.request().clone();
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound()
                .json(json!({
                    "message":
                        format!(
                            "{} is not allowed for url {}",
                            resa.method().to_string(),
                            resa.uri().to_string()
                        )
                }))
                .into_body(),
        ),
    ))
}
