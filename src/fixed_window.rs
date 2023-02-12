use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{Datelike, Timelike, Utc};

use crate::redis_connection::RedisConnection;

pub async fn rate_limiter_redis_fixed_window<B>(
    State(pool_connection): State<RedisConnection>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();

    let rate_limit_key = format!("{year}-{month}-{day}-{hour}-{minute}");

    let mut conn = pool_connection.get().await.unwrap();

    let result: i32 = redis::cmd("INCR")
        .arg(rate_limit_key)
        .query_async(&mut conn)
        .await
        .unwrap();

    if result > 10 {
        Err(StatusCode::TOO_MANY_REQUESTS)
    } else {
        let response = next.run(request).await;
        Ok(response)
    }
}

