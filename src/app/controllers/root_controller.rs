pub async fn root_path_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "routes": ["/", "/status", "/todos"],
        "routes_info": {
            "/" : "this route",
            "/status": "liveness and readiness probes",
            "/todos": "handle todos and more"
        }
    }))
}
