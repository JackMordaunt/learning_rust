#![allow(dead_code)]

/// Iterate over the fibonacci sequence.
struct FibIterator {
    /// limit is the maximum number iterations we allow for this infinite
    /// sequence. 0 indicates no limit. 
    limit: u64,
    index: u64,
    current: u64,
    previous: u64,
}

impl FibIterator {
    fn new(limit: u64) -> FibIterator {
        FibIterator{
            limit,
            index: 0,
            current: 0,
            previous: 0,
        }
    }
}

impl ::std::iter::Iterator for FibIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item >{
        if self.limit != 0 && self.index == self.limit {
            return None;
        }
        self.index += 1;
        let previous = self.previous;
        let current = self.current;
        self.previous = current;
        self.current = match self.current {
            0 => 1,
            _ => current + previous,
        };
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let want: Vec<u64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (ii, n) in FibIterator::new(10).enumerate() {
            assert_eq!(want[ii], n, "want {}, got {}", want[ii], n);
        }
    }

    #[test]
    fn sum() {
        let want: u64 = 88;
        let got = FibIterator::new(10).sum();
        assert_eq!(want, got, "want {}, got {}", want, got);
    }
}
