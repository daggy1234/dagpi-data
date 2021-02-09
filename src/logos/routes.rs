use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;
use serde_json::{Value as JsonValue, json};

use crate::datasets::Logos;

#[get("/logo")]
async fn random_post(data: web::Data<Logos>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &JsonValue = &data.data[post];
    HttpResponse::Ok().json(fjs)
}


#[get("/logo/easy")]
async fn easy_post(data: web::Data<Logos>) -> impl Responder {
    let mut logo = &json!({
        "message": "Error occured"
        });
    let mut found = false;
    while true {
        let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
        let fjs: &JsonValue = &data.data[post];
        if fjs["easy"].as_bool().unwrap() {
            logo = fjs;
            found = true;
            break;
        }
    };
    if found {
        HttpResponse::Ok().json(logo)
    } else {
        HttpResponse::InternalServerError().body("Pain")
    }
    
    
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(random_post);
    config.service(easy_post);
}
