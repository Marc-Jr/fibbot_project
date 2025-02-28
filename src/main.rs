use std::env;
fn main() {
    println!("Hello, world!");

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
    let max_threshold: usize = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap_or(1000);

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
}




