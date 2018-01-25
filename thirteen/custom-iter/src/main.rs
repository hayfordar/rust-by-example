struct Counter {
    count : u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count : 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count < 6 {
            true => Some(self.count),
            false => None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_test() {
        // Pair two counters, skip the first on the second counter,
        // multiply each by pair, toss any results that are not divisible
        // by three
        let sum : u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
