use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

struct BitArray {
    bits: Vec<u8>,
    size: usize,
}

impl BitArray {
    // todo bit array dynamic expansion
    fn new(size: usize) -> Self {
        let num_bytes = (size + 7) / 8;
        BitArray {
            bits: vec![0; num_bytes],
            size,
        }
    }


    // true means 1 and false means 0
    fn set_bit(&mut self, index: usize, value: bool) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if value {
            self.bits[byte_index] |= 1 << bit_index;  // index start from 0_right to left in binary
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
    }

    fn get_bit(&self, index: usize) -> bool {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }
}

type HasherArray = Box<[Box<dyn BuildHasher<Hasher=DefaultHasher> + Send>]>;

pub struct BloomFilter<T: ?Sized + Hash> {
    bit_array: BitArray,
    hasher: HasherArray,
    cap: usize,
    _phantom: PhantomData<T>,
}


impl<T: ?Sized + Hash> BloomFilter<T> {
    pub fn with_option(cap: usize, hashers: HasherArray) -> Self {
        BloomFilter {
            bit_array: BitArray::new(cap),
            hasher: hashers,
            cap,
            _phantom: Default::default(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let v: Vec<Box<dyn BuildHasher<Hasher=DefaultHasher> + Send>> =
            vec![Box::new(RandomState::new())];
        let hash_arr = HasherArray::from(v);
        BloomFilter {
            bit_array: BitArray::new(cap),
            hasher: hash_arr,
            cap,
            _phantom: Default::default(),
        }
    }

    pub fn might_contain(&self, item: &T) -> bool {
        for i in 0..self.hasher.len() {
            let bit_offset = self.calculate_hash(i, item) as usize;
            if !self.bit_array.get_bit(bit_offset) {
                return false;
            }
        }
        true
    }

    pub fn set_item(&mut self, item: &T) {
        for i in 0..self.hasher.len() {
            let bit_offset = self.calculate_hash(i, item) as usize;
            self.bit_array.set_bit(bit_offset, true);
        }
    }

    fn calculate_hash(&self, index: usize, item: &T) -> u64 {
        let mut hasher = self.hasher[index].build_hasher();
        item.hash(&mut hasher);
        hasher.finish() % (self.cap as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut bloom_filter = BloomFilter::with_capacity(10);

        let item1 = "sglk";
        let item2 = "love";
        let item3 = "rust";

        bloom_filter.set_item(&item1);

        assert!(bloom_filter.might_contain(&item1));

        assert!(bloom_filter.might_contain(&item2));

        assert!(!bloom_filter.might_contain(&item3));
    }

    #[test]
    fn test_filter_with_capacity() {
        let mut f: BloomFilter<String> = BloomFilter::with_capacity(102400);
        for x in 0..10000 {
            f.set_item(&x.to_string());
        }

        for x in 5000..15000 {
            if x < 10000 {
                assert!(f.might_contain(&x.to_string()));
            }
        }
    }
}