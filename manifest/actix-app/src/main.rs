use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod models;

use models::Routes;

#[get("/check_fastapi_app")]
async fn check_fastapi_app() -> impl Responder {
    if let Ok(hostname) = std::env::var("FASTAPI_SERVICE_HOSTNAME") {
        let mut routs = Routes::new();

        if models::check_connection(&hostname).await.is_ok() {
            routs
                .update(&hostname)
                .await
                .expect("Error has occured while requesting some route");
        }

        HttpResponse::Ok().body(format!("{:#?}", routs))
    } else {
        HttpResponse::Ok().body(format!("FASTAPI_SERVICE_HOSTNAME variable should be set."))
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Server is working!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(check_fastapi_app))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

// FASTAPI_SERVICE_HOSTNAME="http://localhost:8000" cargo run
