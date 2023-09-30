use axum::{
    body::Bytes,
    routing::get,
    Router,
};
use crate::app::controllers;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

pub fn build() -> Router {
    // Build our CORS layer
    let cors = CorsLayer::new().allow_origin(Any);

    // Build our middleware stack
    let middleware = ServiceBuilder::new()
        // Add high level tracing/logging to all requests
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
        )
        // add cors
        .layer(cors);


    // Build route service
    Router::new()
        .route("/", get(controllers::root_controller::root_path_handler))
        .route("/status/liveness", get(controllers::status_controller::liveness_path_handler))
        .route("/status/readiness", get(controllers::status_controller::readiness_path_handler))
        .route("/todos/:id", get(controllers::todo_controller::get_todo_path_handler))
        .layer(middleware)
}