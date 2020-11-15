use actix_web::{get,web,HttpResponse,Responder};
use crate::datasets::Facts;

use serde_json::json;
use rand::Rng;
#[get("/fact")]
async fn fact(data: web::Data<Facts>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0,&data.list.len()-1);
    let fjs =  &data.list[post];
    HttpResponse::Ok().json(json!(

        {
            "fact":fjs
        }
    )
        
    )
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(fact);
    
}