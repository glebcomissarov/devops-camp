use actix_app::app::{check_fastapi_app, hello};
use actix_web::http::StatusCode;
use actix_web::{body, http::header::ContentType, test, App};

const URI: &str = "/check_fastapi_app?access_token=cloudru125";

#[actix_web::test]
async fn no_hostname_env_var() {
    let app = test::init_service(App::new().service(hello).service(check_fastapi_app)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .uri(URI)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    println!("✅ STATUS CODE: {:#?}", resp.status());

    let resp_body = body::to_bytes(resp.into_body()).await.unwrap();
    println!(
        "💬 RESPONSE BODY: {:#?}",
        std::str::from_utf8(&resp_body).unwrap()
    );
}

#[actix_web::test]
async fn app_just_works() {
    let app = test::init_service(App::new().service(hello).service(check_fastapi_app)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .uri("/")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    println!("✅ STATUS CODE: {:#?}", resp.status());

    let resp_body = body::to_bytes(resp.into_body()).await.unwrap();
    println!(
        "💬 RESPONSE BODY: {:#?}",
        std::str::from_utf8(&resp_body).unwrap()
    );
}

// RUN TESTS
// cargo test --test without_fastapi -- --show-output
