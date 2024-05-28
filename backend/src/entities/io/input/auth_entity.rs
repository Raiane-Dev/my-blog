use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub email: String,
    pub plain_password: String,
}


