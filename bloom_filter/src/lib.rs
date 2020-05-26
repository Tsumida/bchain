//! Implemetation of bloom filter.
//! 
//! ## 原理
//! 布隆过滤器是一种概率性数据结构，提供高效的插入和存在性判断。
//! 插入key时，布隆过滤器使用多个hash函数将key映射到bit数组的若干位bit上，将它们设置为1。
//! 判断某个key是否存在时，先将其hash，再判断这些hash结果是否都为1.如果存在一个0，那么必定不存在。
//! 存在性返回如下两种结果之一:
//! 1. 要查询的key一定不存在
//! 2. 要查询的key可能存在 
//! 返回的查询存在一定误报率(false positive),即该key不存在，但计算结果认为存在。误报率与内部的空间、已使用的比特位有关。一般来说，空间越大、插入的key越少，误报率越低。
//! 布隆过滤器不支持删除操作。
//! 
//! ## 误报率
//! 假设布隆过滤器使用k个hash函数，bit数组长度为m bit, n为已经插入的key个数:
//! 假设k个hash之间是独立的，key之间也是独立的，每个hash的结果都均匀分布。
//! 
//! 某个bit位在一次插入后为0的概率为:
//! `(1-1/m)^k`
//! 
//! 这个bit位在插入n个key后仍然为0的概率为:
//! `(1-1/m)^kn`
//! 
//! 这个bit为在插入n个key后为1的概率为:
//! `[1 - (1-1/m)^kn]`
//! 
//! 那么，k个这样的bit位出现的概率为:
//! `[1 - (1-1/m)^kn]^k`
//! 
//! ## Usage
//! ```rust
//! use bloom_filter::BloomFilterBuilder;
//! 
//! let mut bf = BloomFilterBuilder::new()
//! .set_byte_size(50)
//! .set_hash_steps(&[2, 3, 5, 7])
//! .build().unwrap();
//! 
//! bf.mem_view();
//! 
//! bf.insert("string".as_bytes());
//! bf.mem_view();
//! 
//! bf.insert("hello, world!".as_bytes());
//! bf.mem_view();
//! 
//! assert!(bf.contains("string".as_bytes()));
//! assert!(bf.contains("hello, world!".as_bytes()));
//! 
//! assert!(!bf.contains("helloworld".as_bytes()));
//! ```

mod bf_hasher;
use bf_hasher::simple_hasher;

mod my_bit_vec;
use my_bit_vec::MyBitVec;

pub struct BloomFilterBuilder{
    bit_size: usize, 
    steps: Vec<usize>,
}

impl BloomFilterBuilder{

    pub fn new() -> BloomFilterBuilder{
        BloomFilterBuilder{
            bit_size: 0,
            steps: Vec::new(),
        }
    }

    /// 非精确
    pub fn set_byte_size(&mut self, byte_sz: usize) -> &mut Self{
        self.bit_size = byte_sz;
        self
    }

    pub fn set_hash_steps(&mut self, steps: &[usize])-> &mut Self{
        self.steps = steps.into_iter().cloned().collect();
        self
    }

    pub fn build(&self) -> Option<BloomFilter>{
        if self.steps.len() == 0{
            None
        }else{
            Some(BloomFilter{
                steps: self.steps.clone(),
                num: 0,
                bv: MyBitVec::new(self.bit_size),
            })
        }
        
    }
}

/// BloomFilter.
pub struct BloomFilter{
    steps: Vec<usize>,
    bv: MyBitVec,
    num: usize, 
}

impl BloomFilter{
    pub fn default() -> BloomFilter{
        BloomFilter{
            steps: vec![2, 3, 11],
            num: 0,
            bv: MyBitVec::new(1 << 23), // 1 MB
        }
    }
    pub fn insert(&mut self, key: &[u8]) -> &mut Self{
        let limit = self.bv.bit_size();
        let mut tmp = Vec::new();
        for &step in &self.steps{
            let p = simple_hasher(key, step) % limit;
            tmp.push(p);
            self.bv.set(p);
        }
        let p = tmp.iter().map(|n| (n >> 3, n & 0b111)).collect::<Vec<(usize, usize)>>();
        eprintln!("set {:?}\n-> {:?}", tmp, p);
        self.num += 1;
        self
    }
    
    pub fn contains(&self, key: &[u8]) -> bool{
        let limit = self.bv.bit_size();
        let mut tmp = Vec::new();
        let mut res = true;
        for &step in &self.steps{
            let p = simple_hasher(key, step) % limit;
            tmp.push(p);
            res &= self.bv.contains(p);
        }
        let p = tmp.iter().map(|n| (n >> 3, n & 0b111)).collect::<Vec<(usize, usize)>>();
        eprintln!("check {:?}\n-> {:?}", tmp, p);
        res
    }
    // pub fn false_positive_rate(&self) -> f64{}
    // pub fn size() -> usize{}

    pub fn mem_view(&self){
        eprintln!("============= Bloom Filter =============== ");
        for (i, b) in self.bv.bit_vec.iter().enumerate(){
            eprintln!("index={:4} {:08b}", i, b);
        }
        eprintln!("========================================== ");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bf_usage() {
        let mut bf = BloomFilterBuilder::new()
            .set_byte_size(50)
            .set_hash_steps(&[2, 3, 5, 7])
            .build().unwrap();
    
        bf.mem_view();

        bf.insert("string".as_bytes());
        bf.mem_view();

        bf.insert("hello, world!".as_bytes());
        bf.mem_view();

        assert!(bf.contains("string".as_bytes()));
        assert!(bf.contains("hello, world!".as_bytes()));

        assert!(!bf.contains("helloworld".as_bytes()));
    }
}
