extern crate bit_vec;

use bit_vec::BitVec;
use fasthash::{murmur2, xx};

struct BloomFilter {
    bv: BitVec,
    size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> Self {
        Self {
            bv: BitVec::from_elem(size, false),
            size,
        }
    }

    pub fn add<T: AsRef<[u8]>>(&mut self, item: &T) {
        let xx = xx::hash64(item) as usize % self.size;
        let mm = murmur2::hash64(item) as usize % self.size;

        self.bv.set(xx, true);
        self.bv.set(mm, true);
    }

    pub fn get<T: AsRef<[u8]>>(&self, item: &T) -> bool {
        let xx = xx::hash64(item) as usize % self.size;
        let mm = murmur2::hash64(item) as usize % self.size;

        let xb = self.bv.get(xx).unwrap();
        let mb = self.bv.get(mm).unwrap();

        if xb == true && mb == true {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s1 = String::from("blah blah");
        let s2 = String::from("bleh bleh");
        let s3 = String::from("not in set");

        let mut bf = BloomFilter::new(16);
        bf.add(&s1);
        bf.add(&s2);
        assert_eq!(bf.get(&s1), true);
        assert_eq!(bf.get(&s2), true);
        assert_eq!(bf.get(&s3), false);
    }
}
