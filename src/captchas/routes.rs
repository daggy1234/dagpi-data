use crate::datasets::{Captcha, Captchas};
use actix_web::{get, web, HttpResponse, Responder};
extern crate serde_json;

use rand::Rng;

#[get("/captcha")]
async fn captcha(data: web::Data<Captchas>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &Captcha = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(captcha);
}
