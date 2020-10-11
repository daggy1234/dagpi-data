use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;
use serde_json::Value as JsonValue;

use crate::datasets::WaifuData;

#[get("/waifu")]
async fn waifu_rand(data: web::Data<WaifuData>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &JsonValue = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(waifu_rand);
}
