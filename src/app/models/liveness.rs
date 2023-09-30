use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Liveness {
    pub status: String,
}