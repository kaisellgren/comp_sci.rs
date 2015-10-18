use std::collections::BitvSet;
use std::num::Float;
use std::cmp::max;
use algorithms::murmur::murmur3_32_seed;

pub struct BloomFilter {
    set: BitvSet,
    expected_length: u32,
    hash_count: u32,
}

impl BloomFilter {
    /// Constructs a new bloom filter.
    ///
    /// You must specify the number of bits in the bloom filter, and also you should specify the number of items you
    /// expect to add. The latter is used to choose some optimal internal values to minimize the false-positive
    /// rate (which can be estimated with expected_false_positive_rate()).
    pub fn with_capacity(capacity: u32, expected_length: u32) -> BloomFilter {
        let hash_count = ((capacity / expected_length) as f32 * 2.0.ln()).ceil() as u32;

        BloomFilter {
            set: BitvSet::with_capacity(capacity as usize),
            expected_length: expected_length,
            hash_count: max(1, hash_count),
        }
    }

    /// Returns the expected false-positive rate.
    pub fn expected_false_positive_rate(&self) -> f32 {
        // (1 - e^(-k * n / m)) ^ k
        let pre = (-(self.hash_count as f32) * self.expected_length as f32) / self.set.capacity() as f32;
        (1f32 - pre.exp()).powi(self.hash_count as i32)
    }

    /// Pushes a new value to the bloom filter.
    pub fn push(&mut self, data: &[u8]) {
        let mut hashes = range(0, self.hash_count).map(|i| murmur3_32_seed(data, i) as usize);

        for hash in hashes {
            self.set.insert(hash);
        }
    }

    /// Clears the bloom filter.
    pub fn clear(&mut self) {
        self.set.clear();
    }

    /// Returns false if the data was definitely not added to the bloom filter, and true if it may have been.
    pub fn contains(&self, data: &[u8]) -> bool {
        let hashes = range(0, self.hash_count).map(|i| murmur3_32_seed(data, i) as usize);

        hashes.all(|hash| self.set.contains(&hash))
    }
}

#[test]
fn basic_tests() {
    let mut filter = BloomFilter::with_capacity(10, 5);
    let a = vec![1, 2, 3, 4];
    assert_eq!(false, filter.contains(a.as_slice()));
    filter.push(a.as_slice());
    assert_eq!(true, filter.contains(a.as_slice()));
}
//
#[test]
fn false_positive() {
    let mut filter = BloomFilter::with_capacity(1, 2);
    let a = vec![1, 2, 3, 4];
    assert_eq!(false, filter.contains(a.as_slice()));
    filter.push(a.as_slice());
    assert_eq!(true, filter.contains(a.as_slice()));
    //assert_eq!(true, filter.contains([1u8].as_slice()));
    // TODO: capacity will be increased! maybe MOD?
}
