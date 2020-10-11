use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;

use crate::datasets::PickupLine;
use crate::datasets::PickupLines;

#[get("/pickupline")]
async fn pickup_line(data: web::Data<PickupLines>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &PickupLine = &data.data[post];
    println!("{}", fjs.joke);
    HttpResponse::Ok().json(
        fjs
    )
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(pickup_line);
}