use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hostname {
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodID {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub author: String,
}

#[derive(Debug, Serialize)]
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
                    route.1 = format!("{:?}", send_request::<Hostname>(hostname, &route.0).await);
                }
                "/author" => {
                    route.1 = format!("{:?}", send_request::<Author>(hostname, &route.0).await);
                }
                "/id" => {
                    route.1 = format!("{:?}", send_request::<PodID>(hostname, &route.0).await);
                }
                _ => (),
            }
        }

        Ok(())
    }
}

async fn send_request<T: DeserializeOwned>(hostname: &str, route: &str) -> T {
    let resp: String = reqwest::get(format!("{}{}/", hostname, route))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&resp).unwrap()
}

pub async fn check_connection(hostname: &str) -> Result<(), Error> {
    reqwest::get(hostname).await?;
    Ok(())
}
