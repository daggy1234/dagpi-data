use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;

use crate::datasets::Countries;
use crate::datasets::Country;

#[get("/flag")]
async fn random_flag(data: web::Data<Countries>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &Country = &data.data[post];
    let name = fjs.name.common.to_lowercase().replace(" ", "");
    HttpResponse::Ok().json(
        serde_json::json!({"flag": format!("https://cdn.dagpi.xyz/flags/{}.png",name),"Data":fjs})
    )
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(random_flag);
}
