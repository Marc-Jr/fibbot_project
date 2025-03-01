use regex::Regex;

pub fn extract_numbers(text: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap(); // Matches full numbers
    re.find_iter(text)
        .filter_map(|m| m.as_str().parse::<u32>().ok())
        .collect()
}
