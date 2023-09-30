use crate::app::models::liveness::Liveness;
use crate::app::models::readiness::Readiness;

pub async fn liveness_path_handler() -> axum::Json<serde_json::Value> {
    let liveness = Liveness { status: "Ok".to_string()};

    axum::Json(serde_json::json!(liveness))
    // ou axum::Json(serde_json::to_value(&liveness).unwrap())
}

pub async fn readiness_path_handler() -> axum::Json<serde_json::Value> {
    let readiness = Readiness {
        status: "Ok".to_string(),
        database: "Connected!!!".to_string(),
        cache: "Initialized".to_string(),
    };

    axum::Json(serde_json::json!(readiness))
    // ou axum::Json(serde_json::to_value(&liveness).unwrap())
}