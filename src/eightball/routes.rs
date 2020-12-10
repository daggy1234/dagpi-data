use crate::datasets::EightBall;
use actix_web::{get, web, HttpResponse, Responder};

use rand::Rng;
use serde_json::json;
#[get("/8ball")]
async fn ball(data: web::Data<EightBall>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.list.len() - 1);
    let fjs = &data.list[post];
    HttpResponse::Ok().json(json!({ "response": fjs }))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(ball);
}
