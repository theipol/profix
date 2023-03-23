use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};
use dotenv;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = Response {
        message: "Service up".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let app_port: u16 = dotenv::var("PORT").expect("Not port value").parse::<u16>().unwrap();

    println!("The server is running on port: {}", app_port);

    HttpServer::new(|| App::new().service(health_check).default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", app_port))?
        .run()
        .await
}
