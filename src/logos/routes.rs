use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;
use serde_json::Value as JsonValue;

use crate::datasets::Logos;

#[get("/logo")]
async fn random_post(data: web::Data<Logos>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &JsonValue = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(random_post);
}
