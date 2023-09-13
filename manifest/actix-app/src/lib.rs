mod models;

pub mod req {
    use crate::hash_token::Token;
    use crate::models::{Author, Hostname, PodID};
    use actix_web::http::StatusCode;
    use reqwest::Error;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[derive(Debug, Default, Serialize)]
    pub struct Routes {
        pub data: Vec<(String, String)>,
    }

    impl Routes {
        pub fn new() -> Self {
            Routes {
                data: vec![
                    ("/hostname".to_string(), "Not found".to_string()),
                    ("/author".to_string(), "Not found".to_string()),
                    ("/id".to_string(), "Not found".to_string()),
                ],
            }
        }

        pub async fn update(&mut self, hostname: &str) -> Result<(), Error> {
            for route in self.data.iter_mut() {
                match route.0.as_str() {
                    "/hostname" => {
                        route.1 =
                            format!("{:?}", send_request::<Hostname>(hostname, &route.0).await?);
                    }
                    "/author" => {
                        route.1 =
                            format!("{:?}", send_request::<Author>(hostname, &route.0).await?);
                    }
                    "/id" => {
                        route.1 = format!("{:?}", send_request::<PodID>(hostname, &route.0).await?);
                    }
                    _ => (),
                }
            }

            Ok(())
        }
    }

    async fn send_request<T: DeserializeOwned>(hostname: &str, route: &str) -> Result<T, Error> {
        let resp = reqwest::Client::new()
            .get(format!("{}{}/", hostname, route))
            .send()
            .await?;

        match resp.error_for_status() {
            Ok(resp) => Ok(serde_json::from_str(&resp.text().await?).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// check connection with remote server
    /// by sending http request to root endpoint
    pub async fn check_connection(hostname: &str, token: &str) -> Result<StatusCode, Error> {
        let hashed_token = Token::new(token).get_hash();
        // dbg!(token, hashed_token);

        let resp = reqwest::Client::new()
            .get(hostname)
            .header("REQUEST_TOKEN", hashed_token)
            .send()
            .await?;

        Ok(resp.status())
    }
}

/// Performs token encryption with additional salt value.
pub mod hash_token {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[derive(Hash)]
    pub struct Token {
        token: String,
        salt: String,
    }

    impl Token {
        /// Returns struct `Token` with two string fields: token and salt.
        pub fn new(token: &str) -> Self {
            Token {
                token: token.to_owned(),
                salt: "cloudru".to_string(),
            }
        }

        /// Returns encrypted value based on `Token` struct.
        pub fn get_hash(&self) -> u64 {
            let mut hf = DefaultHasher::new();
            self.hash(&mut hf);

            hf.finish()

            // todo!("REAL-LIFE HASH FUNCTION SHOULD BE ADDED!!!")
        }
    }
}

pub mod app {
    use actix_web::http::StatusCode;
    use actix_web::{get, web, HttpResponse, Responder};
    use serde::Deserialize;

    use crate::req::{self, Routes};

    #[derive(Debug, Deserialize)]
    pub struct Params {
        access_token: String,
    }

    /// Send http request to server with access token.
    /// Endpoint example: `http://hostname/check_fastapi_app?access_token=tokenValue`
    #[get("/check_fastapi_app")]
    pub async fn check_fastapi_app(qparams: web::Query<Params>) -> impl Responder {
        let token = &qparams.access_token;

        if let Ok(hostname) = std::env::var("FASTAPI_SERVICE_HOSTNAME") {
            let mut routs = Routes::new();

            if let Ok(st_code) = req::check_connection(&hostname, token).await {
                if st_code == StatusCode::FORBIDDEN {
                    actix_web::HttpResponseBuilder::new(st_code)
                        .body("Access Denied (incorrect token key).")
                } else if st_code != StatusCode::OK {
                    actix_web::HttpResponseBuilder::new(StatusCode::NOT_IMPLEMENTED)
                        .body("Unknown server response.")
                } else {
                    match routs.update(&hostname).await {
                        Ok(_) => HttpResponse::Ok().body(format!("{:#?}", routs)),
                        Err(e) => {
                            // if app is running in k8s - such response is not expected due to readinessProbe
                            let err_msg =
                                format!("Unknown internal server error ({:?}).", e.status());
                            actix_web::HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(err_msg)
                        }
                    }
                }
            } else {
                actix_web::HttpResponseBuilder::new(StatusCode::SERVICE_UNAVAILABLE)
                    .body("Connection failed: the server is not responding.")
            }
        } else {
            actix_web::HttpResponseBuilder::new(StatusCode::NOT_FOUND)
                .body("Variable FASTAPI_SERVICE_HOSTNAME should be set.")
        }
    }

    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body(format!(
            "Server is working :)\nTry /check_fastapi_app?access_token=tokenValue endpoint!"
        ))
    }
}
