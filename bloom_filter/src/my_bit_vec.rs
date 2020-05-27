
pub struct MyBitVec{
    // length in bit.
    pub bit_size: usize,
    pub bit_vec: Vec<u8>,
}

impl MyBitVec{
    pub fn new(bit_num: usize) -> MyBitVec{
        let mut tmp = bit_num >> 3;
        if (bit_num & 0b111) > 0{
            tmp += 1;
        }

        let mut v = Vec::with_capacity(tmp);
        v.resize(tmp, 0);
        let mbv = MyBitVec{
            bit_size: bit_num,
            bit_vec: v,
        };

        mbv
    }

    #[inline]
    /// Out of Range index is ok.
    pub fn set(&mut self, index: usize) -> &Self{
        if let Some(v) = self.bit_vec.get_mut(index >> 3){
            *v |= 1 << (index & 0b111);
        }
        self
    }

    #[inline]
    /// For out of range index, return false.
    pub fn contains(&self, index: usize) -> bool{
        match self.bit_vec.get(index >> 3){
            None => false,
            Some(i) => (i >> (index & 0b111)) & 1 == 1,
        }
    }

    #[inline]
    /// Set many times.
    pub fn multi_set(&mut self, indices: &[usize]) -> &Self{
        indices.iter().for_each(|i| {self.set(*i);} );
        self
    }

    #[inline]
    pub fn multi_check(&mut self, indices: &[usize]) -> bool{
        indices.iter().fold(true, |total, x| total & self.contains(*x))
    }

    #[inline]
    pub fn bit_size(&self) -> usize{
        self.bit_size
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.bit_vec.len()
    }
}

#[cfg(test)]
mod mbv_test{
    use super::*;

    #[test]
    fn mbv_usage() {

        assert_eq!(120 >> 3, MyBitVec::new(120).len());
        let mut mbv = MyBitVec::new(121); // 121 bit
        assert!(mbv.len() == (120 >> 3) + 1); // 16 Bytes.
        assert!(mbv.bit_size() == 121);

        mbv.set(12);
        assert_eq!(*mbv.bit_vec.get(1).unwrap(), (1 << 4));
        mbv.set(13);
        assert_eq!(*mbv.bit_vec.get(1).unwrap(),(1 << 4) | (1 << 5));
        mbv.set(13); // 幂等
        assert_eq!(*mbv.bit_vec.get(1).unwrap(),(1 << 4) | (1 << 5));
        mbv.set(1024); // out of range.

        assert!(mbv.contains(12));
        assert!(mbv.contains(13));
        assert!(!mbv.contains(14));

        assert!(mbv.multi_check(&[12, 13]));
        assert!(!mbv.multi_check(&[12, 13, 14]));
    }
}

