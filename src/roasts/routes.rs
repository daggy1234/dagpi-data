use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;
use serde_json::json;

use crate::datasets::Roasts;

#[get("/roast")]
async fn roast(data: web::Data<Roasts>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.list.len() - 1);
    let fjs = &data.list[post];
    HttpResponse::Ok().json(json!({ "roast": fjs }))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(roast);
}
