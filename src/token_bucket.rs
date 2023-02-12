use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct TokenBucket {
    max_tokens: usize,
    refill_rate: Duration,
    available_tokens: Arc<AtomicUsize>,
    last_refill_timestamp: DateTime<Utc>,
}

impl TokenBucket {
    pub fn new() -> Self {
        Self {
            max_tokens: 10,
            refill_rate: Duration::minutes(10),
            available_tokens: Arc::new(10.into()),
            last_refill_timestamp: Utc::now(),
        }
    }

    fn allow(&mut self, tokens: usize) -> bool {
        self.refill();

        dbg!(&self.available_tokens);

        if self.available_tokens.load(Ordering::Relaxed) >= tokens {
            self.available_tokens.fetch_sub(tokens, Ordering::SeqCst);
            return true;
        }
        return false;
    }

    fn refill(&mut self) {
        let now = Utc::now();
        let time_since_last_refill = now - self.last_refill_timestamp;
        let number_of_tokens_to_add =
            time_since_last_refill.num_minutes() / self.refill_rate.num_minutes();

        // BAD: casting i64 to usize can lead to issues
        let number_of_tokens_to_add =
            number_of_tokens_to_add.min(self.max_tokens.try_into().unwrap());

        // BAD: casting i64 to usize can lead to issues
        self.available_tokens
            .fetch_add(number_of_tokens_to_add as usize, Ordering::SeqCst);
        self.last_refill_timestamp = now;
    }
}

pub async fn rate_limiter_token_bucket<B>(
    State(mut token_bucket): State<TokenBucket>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let request_cost = 1;

    if token_bucket.allow(request_cost) {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}
