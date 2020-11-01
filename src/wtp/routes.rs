extern crate reqwest;
extern crate serde_json;

use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;

use crate::datasets::BasicMon;
use crate::datasets::MonVec;


#[get("/wtp")]
async fn random_post(data: web::Data<MonVec>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.list.len() - 1);
    let fjs: &BasicMon = &data.list[post];
    return HttpResponse::Ok().json(
        serde_json::json!({"question": String::from(format!("https://logoassetsgame.s3.us-east-2.amazonaws.com/wtp/pokemon/{}q.png",fjs.id)),"answer": String::from(format!("https://logoassetsgame.s3.us-east-2.amazonaws.com/wtp/pokemon/{}a.png",fjs.id)),"Data":fjs})
    );
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(random_post);
}
