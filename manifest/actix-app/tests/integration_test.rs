// cargo test --test integration_test -- --show-output

// ADD MORE TESTS !!!

use actix_app::app::{check_fastapi_app, hello};
use actix_web::http::StatusCode;
use actix_web::{body, http::header::ContentType, test, App};

const URI: &str = "/check_fastapi_app?access_token=cloudru125";

#[actix_web::test]
async fn fastapi_is_not_responding() {
    // set env variable
    std::env::set_var("FASTAPI_SERVICE_HOSTNAME", "http://localhost:8000");

    let app = test::init_service(App::new().service(hello).service(check_fastapi_app)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .uri(URI)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    println!("✅ STATUS CODE: {:#?}", resp.status());

    let resp_body = body::to_bytes(resp.into_body()).await.unwrap();
    println!(
        "💬 RESPONSE BODY: {:#?}",
        std::str::from_utf8(&resp_body).unwrap()
    );
}
