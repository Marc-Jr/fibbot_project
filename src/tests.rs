#[cfg(test)]
mod tests {
    use crate::extract::extract_numbers;

    #[test]
    fn test_extract_numbers() {
        let text = "Values: 123, 456, and 789.";
        let numbers = extract_numbers(text);
        assert_eq!(numbers, vec![123, 456, 789]);
    }



    mod tests {
        use crate::fibonacci::Fibonacci;
    
        #[test]
        fn test_fibonacci() {
            let mut fib_calc = Fibonacci::new();
            assert_eq!(fib_calc.compute(5), 5);
            assert_eq!(fib_calc.compute(10), 55);
        }
    }
}
    
    