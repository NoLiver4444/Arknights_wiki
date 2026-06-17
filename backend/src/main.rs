use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use crate::data::test_data;

mod models;
mod data;
mod routes;
mod handlers;
mod cache;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Предзагрузка данных при старте сервера
    println!("Preloading operators data...");
    match handlers::operator_handler::fetch_operators_from_api().await {
        Ok(operators) => {
            println!("Loaded {} operators", operators.len());
            cache::set_operators(operators).await;
        }
        Err(e) => {
            eprintln!("Failed to preload operators: {}", e);
            println!("Server will start anyway. Data will be loaded on first request.");
        }
    }
    
    println!("Starting server on 127.0.0.1:8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(routes::operator_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
