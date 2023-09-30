use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Readiness {
    pub status: String,
    pub database: String,
    pub cache: String,
}