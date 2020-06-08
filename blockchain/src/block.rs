//! Blocks, BlockHeaders, 
pub trait HashVal: Default{
}

#[derive(Debug, Clone)]
pub struct BlockHeader<H: HashVal>{
    version: u32,        // 4B
    prev_block: H,       // 32B
    merkle_root: H,      // 32B
    timestamp: u64,      // 64
    bits: u32,          
    nonce: u32, 
}

impl<H: HashVal> BlockHeader<H>{
    pub fn set_merkle_root(&mut self, mr: H) -> &mut Self{
        self.merkle_root = mr;
        self
    }

    pub fn update_timestamp(&mut self, ts: u64) -> &mut Self{
        self.timestamp = u64::max(self.timestamp, ts);
        self
    }

}

#[derive(Clone, Debug)]
pub struct Block<H: HashVal>{
    header: BlockHeader<H>,
    // data
}

impl<H: HashVal> Block<H>{
    pub fn genesis_block(ts: u64) -> Block<H>{
        Block{
            header: BlockHeader{
                version: 0,
                prev_block: H::default(),
                merkle_root: H::default(),
                timestamp: ts,
                bits: 0,
                nonce: 0,
            }
        }
    }
}

