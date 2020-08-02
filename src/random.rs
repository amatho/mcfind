use std::{iter, num::Wrapping};

const MULTIPLIER: Wrapping<i64> = Wrapping(0x5DEECE66D);
const ADDEND: Wrapping<i64> = Wrapping(0xB);
const MASK: Wrapping<i64> = Wrapping((1 << 48) - 1);

pub struct JavaRandom {
    state: Wrapping<i64>,
}

impl JavaRandom {
    pub fn new(seed: i64) -> Self {
        JavaRandom {
            state: Wrapping((seed ^ MULTIPLIER.0) & MASK.0),
        }
    }

    pub fn next(&mut self, bits: u8) -> i32 {
        assert!(bits <= 48, "bits must be less or equal 48");

        self.state = (self.state * MULTIPLIER + ADDEND) & MASK;

        (self.state.0 as u64 >> (48 - bits)) as i32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next(32) as u32
    }

    pub fn next_i32_bound(&mut self, bound: u32) -> i32 {
        const MAX_BOUND: u32 = (i32::MAX as u32) + 1;
        assert!(!(bound > MAX_BOUND));

        if bound.is_power_of_two() {
            let bound = bound as i64;
            (bound.wrapping_mul(self.next(31) as i64) >> 31) as i32
        } else {
            let bound = bound as i32;
            let max = bound - 1;

            iter::repeat_with(|| self.next(31))
                .find(|v| v.wrapping_sub(v % bound).wrapping_add(max) >= 0)
                .map(|v| v % bound)
                .unwrap()
        }
    }

    pub fn next_u32_bound(&mut self, bound: u32) -> u32 {
        self.next_i32_bound(bound) as u32
    }

    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) | (self.next(32) as i64)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.next_i64() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for parity with Java implementation

    #[test]
    fn next_i32_parity() {
        let mut rnd = JavaRandom::new(42);

        assert_eq!(rnd.next_i32(), -1170105035);
        assert_eq!(rnd.next_i32(), 234785527);
    }

    #[test]
    fn next_i32_bound_parity() {
        let mut rnd = JavaRandom::new(42);

        assert_eq!(rnd.next_i32_bound(420), 50);
        assert_eq!(rnd.next_i32_bound(420), 243);
    }

    #[test]
    fn next_i64_parity() {
        let mut rnd = JavaRandom::new(42);

        assert_eq!(rnd.next_i64(), -5025562857975149833);
        assert_eq!(rnd.next_i64(), -5843495416241995736);
    }
}
