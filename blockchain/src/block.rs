//! Blocks, BlockHeaders, 
//! 
//! 

use merkletree::merkle::{MerkleTree};
use merkletree::store::VecStore;

//use digest::{Input, FixedOutput};
//use sha2::Sha256;

use crate::sym_def::{
    HashVal,
    HashAlgorithm,
};
use crate::transaction::{Transaction, TxAddr, CoinValue};

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
    mkt: MerkleTree<HashVal, HashAlgorithm, merkletree::store::VecStore<HashVal>>, //
}
    
#[derive(Clone, Debug)]
pub struct Block{
    header: BlockHeader,
    data: BlockData,
    // data
}

impl Block{
    /// Create a genesis_block.
    pub fn genesis_block(ts: u64) -> Block{
        let mkt = MerkleTree::new(vec![HashVal([0; 32]); 2]).unwrap();
        Block{
            header: BlockHeader{
                version: 0,
                prev_block: [0; 32],
                merkle_root: mkt.root().0,
                timestamp: ts,
                bits: 0,
                nonce: 0,
            },
            data: BlockData{
                mkt: mkt,
            }
        }
    }

    /// Pack transactions into a block. Panic if txs has length n != 2^k for any k >= 1 .
    pub fn pack<A, V>(ts: u64, txs: impl IntoIterator<Item=Transaction<A, V>>) -> Block
        where A: TxAddr + AsRef<[u8]>, V: CoinValue 
    {
        let hashes = txs.into_iter().map(|tx| tx.into()).collect::<Vec<HashVal>>();
        assert!(hashes.len().is_power_of_two());
        let mkt: MerkleTree<HashVal, HashAlgorithm, VecStore<_>> = MerkleTree::new(hashes).unwrap();

        Block{
            header: BlockHeader{
                version: 0,
                prev_block: [0; 32],
                merkle_root: mkt.root().0,
                timestamp: ts,
                bits: 0,
                nonce: 0,
            },
            data: BlockData{
                mkt: mkt,
            }
        }
    }

    /// Return merkleroot of the block.
    pub fn mkt_root(&self) -> HashVal{
        self.data.mkt.root()
    }

}

