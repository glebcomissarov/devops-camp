use actix_app::app::{check_fastapi_app, hello};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(check_fastapi_app))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

// RUN LOCALLY:
// localhost:8080/check_fastapi_app?access_token=cloudru125
// FASTAPI_SERVICE_HOSTNAME="http://localhost:8000" cargo run

// ADD TESTS !!!
