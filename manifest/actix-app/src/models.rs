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
