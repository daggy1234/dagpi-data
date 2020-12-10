use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;
use serde_json::Value as JsonValue;

use crate::datasets::WaifuData;

#[get("/waifu")]
async fn waifu_rand(data: web::Data<WaifuData>) -> impl Responder {
    let post = rand::thread_rng().gen_range(0, &data.data.len() - 1);
    let fjs: &JsonValue = &data.data[post];
    HttpResponse::Ok().json(fjs)
}

#[get("/waifu/{query}")]
async fn waifu_query(query: web::Path<String>, data: web::Data<WaifuData>) -> impl Responder {
    let mut found: bool = false;
    let mut val = &serde_json::json!({"name": "sex"});
    let str_vex = &data
        .data
        .iter()
        .map(|v| -> String {
            let v_slug = v["slug"].to_string().to_lowercase().replace("\"", "");
            if v_slug == query.to_lowercase() {
                found = true;
                val = v;
            }
            v_slug
        })
        .collect::<Vec<String>>();
    let v8: Vec<&str> = str_vex.iter().map(AsRef::as_ref).collect();
    if found {
        HttpResponse::Ok().json(val)
    } else {
        let resp = difflib::get_close_matches(&query, v8, 5, 0.6);
        HttpResponse::BadRequest().json(serde_json::json!({ "alternatives": resp }))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(waifu_rand);
    config.service(waifu_query);
}
