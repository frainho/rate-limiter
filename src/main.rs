use std::net::SocketAddr;

use axum::{middleware, routing::get, Router};
use fixed_window::rate_limiter_redis_fixed_window;
use redis_connection::RedisConnection;
use token_bucket::{rate_limiter_token_bucket, TokenBucket};

mod fixed_window;
mod redis_connection;
mod token_bucket;

#[tokio::main]
async fn main() {
    let connection_pool = match RedisConnection::new() {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let token_bucket = TokenBucket::new();

    let app = Router::new()
        .route("/", get(root))
        .route_layer(middleware::from_fn_with_state(
            connection_pool.clone(),
            rate_limiter_redis_fixed_window,
        ))
        .route_layer(middleware::from_fn_with_state(
            token_bucket.clone(),
            rate_limiter_token_bucket,
        ))
        .with_state(connection_pool)
        .with_state(token_bucket);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
