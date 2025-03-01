pub fn format_fibonacci_response(numbers: &[u32], fib_results: &[u32]) -> String {
    numbers.iter()
        .zip(fib_results)
        .map(|(num, fib)| format!("{} → {}", num, fib))
        .collect::<Vec<_>>()
        .join(", ")
}
