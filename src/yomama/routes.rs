use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;

use crate::datasets::YoDataset;
use crate::datasets::YoMamaJoke;

#[get("/yomama")]
async fn yomama_joke(data: web::Data<YoDataset>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &YoMamaJoke = &data.data[post];
    HttpResponse::Ok().json(
        fjs
    )
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(yomama_joke);
}