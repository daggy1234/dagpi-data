use crate::datasets::Headlines;
use actix_web::{get, web, HttpResponse, Responder};
extern crate serde_json;
use crate::datasets::Headline;
use rand::Rng;

#[get("/headline")]
async fn headline(data: web::Data<Headlines>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.headlines.len() - 1);
    let fjs: &Headline = &data.headlines[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(headline);
}
