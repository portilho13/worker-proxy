use std::{cmp, time::SystemTime};


const MAX_BUCKET_SIZE: f32 = 3.0;
const REFILL_RATE: f32 = 1.0;

pub struct TokenBucket {
    current_bucket_size: f32,
    last_refill_time: u128,
}

impl TokenBucket {
    pub fn new(current_bucket_size: f32, last_refill_time: u128) -> Self {
        TokenBucket {
            current_bucket_size,
            last_refill_time,
        }
    }

    pub fn allow_request(&mut self, tokens: f32) -> bool {
        self.refill();

        if self.current_bucket_size >= tokens {
            self.current_bucket_size -= tokens;
            return true;
        }
        false
    }

    pub fn refill(&mut self) {
        let time = get_time();
        let elapsed_time = (time - self.last_refill_time) as f32;
        let tokens_to_add = (elapsed_time / 1e9) * REFILL_RATE; // Refill rate per second
        println!("Tokens to add: {}", tokens_to_add);

        self.current_bucket_size = (self.current_bucket_size + tokens_to_add).min(MAX_BUCKET_SIZE);
        self.last_refill_time = time;
    }
}

fn get_time() -> u128 {
    let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    duration.as_nanos()
}
