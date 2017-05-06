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
    /// Create a new BitSet with *ZERO* bit.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new BitSet with `nbits` bits with all bit initialized by `0`.
    ///
    /// # Arguments
    ///
    /// * `nbits` - A integer, which value is the bits count `BitSet` will hold.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::with_capacity(100);
    /// ```
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

    /// Create a new BitSet from a `u64` value, and initialize all bits by `0`.
    ///
    /// # Arguments
    ///
    /// * `v` - A `u64` value.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::from_u64(2);
    /// assert!(bs.test(0) == false);
    /// assert!(bs.test(1) == true);
    /// ```
    pub fn from_u64(v: u64) -> Self {
        BitSet {
            bits: vec![v],
            nbits: 64
        }
    }

    /// Create a new BitSet from a `u64` `vec`, and initialize all bits by `0`.
    ///
    /// # Arguments
    ///
    /// * `vec` - A `u64` vector.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let vec = vec![u64::max_value(), 0, u64::max_value()];
    /// let bs = BitSet::from_vec64(&vec);
    /// assert!(bs.test(63) == true);
    /// assert!(bs.test(64) == false);
    /// ```
    pub fn from_vec64(vec: &Vec<u64>) -> Self {
        BitSet {
            bits: vec.to_vec(),
            nbits: vec.len() * 64
        }
    }

    /// Return the actual bits count.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    /// 
    /// let bs = BitSet::with_capacity(100);
    /// assert!(bs.size() == 100);
    /// ```
    pub fn size(&self) -> usize {
        self.nbits
    }

    /// Return the count of `1`.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::with_capacity(100);
    /// assert!(bs.count() == 0);
    /// ```
    pub fn count(&self) -> u64 {
        (0..self.blocks()).map(|n| bit_count64(self.bits[n]))
                                .fold(0, |sum, i| sum + i)
    }

    /// Return if the given bit index has been set to `1`.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::with_capacity(100);
    /// assert!(bs.test(99) == false);
    /// ```
    pub fn test(&self, bit_idx: usize) -> bool {
        let (block_idx, mod_bit_idx) = (bit_idx / 64, bit_idx % 64);
        let n: u64 = self.bits[block_idx];
        (n >> mod_bit_idx) & 0x1 == 0x1
    }

    /// Return if there is one bit has been set to `1` in the whole bitset..
    ///
    /// # Example
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::with_capacity(100);
    /// assert!(bs.any() == false);
    /// ```
    pub fn any(&self) -> bool {
        for i in 0..self.blocks() {
            if self.bits[i] != 0 {
                return true;
            }
        }
        false
    }

    /// Return if all bits are set to `0`.
    ///
    /// # Example
    /// ```
    /// use bitset::BitSet;
    ///
    /// let bs = BitSet::with_capacity(100);
    /// assert!(bs.none() == true);
    /// ```
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
    /// Set the bit specified by `bit_idx` to `v`, which is `true` or `false`.
    ///
    /// # Arguments
    ///
    /// * `bit_idx` - the bit index we want to set.
    /// * `v` - the value we want to set. `true` or `false`.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let mut bs = BitSet::with_capacity(100);
    /// bs.set(99, true);
    /// assert!(bs.test(99) == true);
    /// ```
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

    /// Reset all bits to `0`.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let mut bs = BitSet::with_capacity(100);
    /// bs.set(99, true);
    /// assert!(bs.test(99) == true);
    /// bs.reset();
    /// assert!(bs.test(99) == false);
    /// ```
    pub fn reset(&mut self) {
        for i in 0..self.blocks() {
            self.bits[i] = 0;
        }
    }

    /// Flip the bit specified by `bit_idx` to the reverse value.
    /// If the bit value is `true`, then it will be flipped to `false`.
    /// The other case is like the same.
    ///
    /// # Arguments
    ///
    /// `bit_idx` - the index of the bit we want to flip.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let mut bs = BitSet::with_capacity(100);
    /// assert!(bs.test(99) == false);
    /// bs.flip(99);
    /// assert!(bs.test(99) == true);
    /// ```
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

    /// Flip all bits in the bitset. It may run time-costly.
    ///
    /// # Example
    ///
    /// ```
    /// use bitset::BitSet;
    ///
    /// let mut bs = BitSet::with_capacity(100);
    /// bs.flip_all();
    /// for i in 0..100 {
    ///     assert!(bs.test(i) == true);
    /// }
    /// bs.flip_all();
    /// for i in 0..100 {
    ///     assert!(bs.test(i) == false);
    /// }
    /// ```
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
