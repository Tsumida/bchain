//! Blocks, BlockHeaders, 
//! 
//! 

use merkletree::merkle::{MerkleTree, Element};
use digest::{Input, FixedOutput};
use sha2::Sha256;

use crate::mkt::HashAlgorithm;

#[derive(Debug, Clone)]
pub struct BlockHeader{
    version: u32,        // 4B
    prev_block: [u8; 32],       // 32B
    merkle_root: [u8; 32],      // 32B
    timestamp: u64,      // 64
    bits: u32,          
    nonce: u32, 
}

impl BlockHeader{
    pub fn set_merkle_root(&mut self, mr: [u8; 32] ) -> &mut Self{
        self.merkle_root = mr;
        self
    }

    pub fn update_timestamp(&mut self, ts: u64) -> &mut Self{
        self.timestamp = u64::max(self.timestamp, ts);
        self
    }
}

#[derive(Clone, Debug)]
pub struct BlockData{
    merkle_tree: MerkleTree<[u8; 32], HashAlgorithm, merkletree::store::VecStore<[u8; 32]>>, //
}

#[derive(Clone, Debug)]
pub struct Block{
    header: BlockHeader,
    // data: BlockData,
    // data
}

impl Block{
    pub fn genesis_block(ts: u64) -> Block{
        Block{
            header: BlockHeader{
                version: 0,
                prev_block: [0; 32],
                merkle_root: [0; 32],
                timestamp: ts,
                bits: 0,
                nonce: 0,
            }
        }
    }
}

