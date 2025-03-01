use std::collections::HashMap;

pub struct Fibonacci {
    cache: HashMap<u32, u32>,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            cache: HashMap::new(),
        }
    }

    pub fn compute(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if let Some(&val) = self.cache.get(&n) {
            return val;
        }
        let result = self.compute(n - 1) + self.compute(n - 2);
        self.cache.insert(n, result);
        result
    }
}

