use once_cell::sync::OnceCell;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crate::models::Operator;

struct CachedData {
    operators: Vec<Operator>,
    fetched_at: Instant,
}

static CACHE: OnceCell<Mutex<CachedData>> = OnceCell::new();
const CACHE_DURATION: Duration = Duration::from_secs(3600 * 24);

pub async fn get_operators() -> Option<Vec<Operator>> {
    let cache = CACHE.get()?;
    let data = cache.lock().await;
    
    if data.fetched_at.elapsed() < CACHE_DURATION {
        Some(data.operators.clone())
    } else {
        None
    }
}

pub async fn set_operators(operators: Vec<Operator>) {
    let data = CachedData {
        operators,
        fetched_at: Instant::now(),
    };
    
    match CACHE.get() {
        Some(mutex) => {
            let mut cache = mutex.lock().await;
            *cache = data;
        }
        None => {
            let _ = CACHE.set(Mutex::new(data));
        }
    }
}