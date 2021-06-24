use crate::datasets::{Typeracer, TyperacerSentence};
use actix_web::{get, web, HttpResponse, Responder};
extern crate serde_json;

use rand::Rng;

#[get("/typeracer")]
async fn typeracer(data: web::Data<Typeracer>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &TyperacerSentence = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(typeracer);
}
