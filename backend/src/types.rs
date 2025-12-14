use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetRequest {
    pub key: String,
}

#[derive(Deserialize)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
}