// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Simple bitset library like C++.

pub struct BitSet {
    bits:   Vec<u64>,
    nbits: usize,
}

impl Default for BitSet {
    #[inline]
    fn default() -> Self { BitSet { bits: Vec::new(), nbits: 0 } }
}

fn bit_count64(u: u64) -> u64 {
    let mut x: u64 = u - ((u >> 1) & 0x5555555555555555u64);
    x = (x & 0x3333333333333333u64) + ((x >> 2) & 0x3333333333333333u64);
    (((x + (x >> 4)) & 0xF0F0F0F0F0F0F0Fu64).wrapping_mul(0x101010101010101u64)) >> 56
}

// Private functions
impl BitSet {
    fn blocks(&self) -> usize {
        if self.nbits % 64 == 0 {
            self.nbits / 64
        }
        else {
            self.nbits / 64 + 1
        }
    }
}

// Public functions
impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(nbits: usize) -> Self {
        let mut bitset = BitSet {
            bits: Vec::new(),
            nbits: nbits
        };
        for _ in 0..bitset.blocks() {
            bitset.bits.push(0);
        }
        bitset
    }

    pub fn size(&self) -> usize {
        self.nbits
    }

    pub fn count(&self) -> u64 {
        (0..self.blocks()).map(|n| bit_count64(self.bits[n]))
                                .fold(0, |sum, i| sum + i)
    }

    pub fn test(&self, bit_idx: usize) -> bool {
        let (block_idx, mod_bit_idx) = (bit_idx / 64, bit_idx % 64);
        let n: u64 = self.bits[block_idx];
        (n >> mod_bit_idx) & 0x1 == 0x1
    }

    pub fn any(&self) -> bool {
        for i in 0..self.blocks() {
            if self.bits[i] != 0 {
                return true;
            }
        }
        false
    }

    pub fn none(&self) -> bool {
        !self.any()
    }

    // // bit vec operations
    // fn union(&mut self, vec: &Vec<u64>) {
    //     //TODO
    // }

    // fn intersect(&mut self, vec: Vec<u64>) {
    //     //TODO
    // }

    // bit operations
    pub fn set(&mut self, bit_idx: usize, v: bool) {
        let (block_idx, mod_bit_idx) = (bit_idx / 64, bit_idx % 64);
        if let Some(n) = self.bits.get_mut(block_idx) {
            if v {
                *n |= 0x1 << mod_bit_idx;
            }
            else {
                *n &= !(0x1 << mod_bit_idx);
            }
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.blocks() {
            self.bits[i] = 0;
        }
    }

    pub fn flip(&mut self, bit_idx: usize) {
        let (block_idx, mod_bit_idx) = (bit_idx / 64, bit_idx % 64);
        if let Some(n) = self.bits.get_mut(block_idx) {
            if (*n >> mod_bit_idx) & 0x1 == 0x1 {
                *n &= !(0x1 << mod_bit_idx);
            }
            else {
                *n |= 0x1 << mod_bit_idx;
            }
        }
    }

    pub fn flip_all(&mut self) {
        for i in 0..self.blocks() {
            self.bits[i] = !self.bits[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_count64() {
        assert!(bit_count64(0) == 0);
        assert!(bit_count64(1) == 1);
        assert!(bit_count64(3) == 2);
        assert!(bit_count64(u64::max_value()) == 64);
    }

    #[test]
    fn test_new() {
        let bitset = BitSet::new();
        assert!(bitset.count() == 0);
        assert!(bitset.size() == 0);
    }

    #[test]
    fn test_with_capacity() {
        let mut bitset = BitSet::with_capacity(100);

        // set/reset/test
        assert!(bitset.test(99) == false);
        bitset.set(0, true);
        assert!(bitset.test(0) == true);
        bitset.set(0, false);
        assert!(bitset.test(0) == false);
        bitset.set(99, true);
        assert!(bitset.test(99) == true);
        bitset.set(99, false);
        assert!(bitset.test(99) == false);
        bitset.reset();
        bitset.set(0, true);
        assert!(bitset.test(0) == true);

        // any/none
        bitset.reset();
        assert!(bitset.any() == false);
        assert!(bitset.none() == true);
        bitset.flip(0);
        assert!(bitset.any() == true);
        assert!(bitset.none() == false);

        // flip
        bitset.reset();
        bitset.set(99, true);
        assert!(bitset.test(99) == true);
        bitset.flip(99);
        assert!(bitset.test(99) == false);
        bitset.flip(99);
        assert!(bitset.test(99) == true);

        // flip_all
        bitset.reset();
        bitset.flip_all();
        for i in 0..100 {
            assert!(bitset.test(i) == true);
        }
        bitset.flip_all();
        for i in 0..100 {
            assert!(bitset.test(i) == false);
        }
    }
}
