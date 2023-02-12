use axum::{middleware, routing::get, Router};
use rate_limiter::rate_limiter;
use redis_connection::RedisConnection;
use std::net::SocketAddr;

mod rate_limiter;
mod redis_connection;

#[tokio::main]
async fn main() {
    let connection_pool = match RedisConnection::new() {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    let app = Router::new()
        .route("/", get(root))
        .route_layer(middleware::from_fn_with_state(
            connection_pool.clone(),
            rate_limiter,
        ))
        .with_state(connection_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
