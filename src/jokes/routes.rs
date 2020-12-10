use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;

use crate::datasets::Joke;
use crate::datasets::Jokes;

#[get("/joke")]
async fn joke(data: web::Data<Jokes>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &Joke = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(joke);
}
