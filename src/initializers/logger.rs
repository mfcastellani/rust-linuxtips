use std::env;

fn log_level() -> tracing::Level {
    match env::var("RUST_LOG") {
        Ok(value) => {
            match value.as_str() {
                "debug" => tracing::Level::DEBUG,
                "error" => tracing::Level::ERROR,
                "info" => tracing::Level::INFO,
                "trace" => tracing::Level::TRACE,
                "warn" => tracing::Level::WARN,
                _ => tracing::Level::ERROR,
            }
        },
        Err(_e) => {
            tracing::Level::INFO
        }
    }
}

pub fn start_logger() {
    tracing_subscriber::fmt().with_max_level(log_level()).init();
}