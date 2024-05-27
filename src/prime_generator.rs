pub struct PrimeGenerator {
    pub current: usize,
}

impl PrimeGenerator {
    pub fn from(current: usize) -> PrimeGenerator {
        PrimeGenerator { current }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as usize) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = self.current;
        while !self.is_prime(num) {
            num += 1;
        }
        self.current = num + 1;
        Some(num)
    }
}
