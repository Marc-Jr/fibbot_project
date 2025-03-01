use std::env;

pub struct Config {
    pub enable_fib: bool,
    pub max_threshold: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let enable_fib = env::var("ENABLE_FIB")
            .unwrap_or_else(|_| "true".to_string())
            == "true";
        let max_threshold = env::var("MAX_THRESHOLD")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .unwrap_or(1000);

        Config { enable_fib, max_threshold }
    }
}