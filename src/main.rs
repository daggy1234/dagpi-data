use actix_web::{App, HttpRequest, HttpServer, middleware, Responder, web};
//use dotenv;
use env_logger;
use reqwest;
use actix_web_prom::PrometheusMetrics;
use std::collections::HashMap;
mod datasets;

mod error;
mod jokes;
mod logos;
mod middlewares;
mod pickup;
mod roasts;
mod waifus;
mod wtp;
mod yomama;

mod handler;
//use actix_files as fs;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Removed for production re-add when testing
    //dotenv::dotenv().unwrap();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    let start = std::env::var("URL").expect("WE NEED A URL ");
    let port = std::env::var("PORT").expect("WE NEED A port ");
    let client = reqwest::Client::new();
    let mut labels = HashMap::new();
    labels.insert("api".to_string(), "Dagpi-Data".to_string());
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .data(client.clone())
            .data(datasets::mondata())
            .data(datasets::jokedata())
            .data(datasets::yomamadata())
            .data(datasets::waifudata())
            .data(datasets::linegen())
            .data(datasets::roasts())
            .data(datasets::logodata())
            .wrap(middlewares::RequiresAuth)
            .wrap(prometheus.clone())
            .configure(wtp::init_routes)
            .configure(yomama::init_routes)
            .configure(pickup::init_routes)
            .configure(waifus::init_routes)
            .configure(jokes::init_routes)
            .configure(roasts::init_routes)
            .configure(logos::init_routes)
            .default_service(web::route().to(error::resp_not_found))
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(format!("{}:{}", start, port))?
        .run()
        .await
}
