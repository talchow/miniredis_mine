use serde::Serialize;
#[derive(Serialize)] // Deserialize for requests
pub struct GetRequest {
    pub key: String,
}

#[derive(Serialize)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
}