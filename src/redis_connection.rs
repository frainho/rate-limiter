use std::ops::{Deref, DerefMut};

use deadpool_redis::{Config, Pool, Runtime};

#[derive(Clone)]
pub struct RedisConnection {
    connection_pool: Pool,
}

impl Deref for RedisConnection {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.connection_pool
    }
}

impl DerefMut for RedisConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection_pool
    }
}

impl RedisConnection {
    pub fn new() -> redis::RedisResult<Self> {
        let cfg = Config::from_url("redis://127.0.0.1/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

        Ok(Self {
            connection_pool: pool,
        })
    }
}
