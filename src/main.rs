mod app;
mod initializers;

use initializers::{
    // database,
    logger,
};

#[tokio::main]
async fn main() {
    logger::start_logger();

    // let pool = database::database_pool();

    // run it with hyper on localhost:3000
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app::router::build().into_make_service())
        .await
        .unwrap();
}
