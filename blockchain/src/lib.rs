//! # 概述
//! 模仿BTC的架构实现。
//! 
//! 首先BTC本质上是一个分布式状态机，状态即UTXO的集合：
//! 
//! ```
//!                          
//!     Net：               P2P, Gossip
//!                             |
//!                         BlockChecker
//!                              |
//!                            UXTO
//!     Mem:               | MemPool   |------- Mining(Pow, Diff, )
//!                        |     |     |
//!     Stablization :     | KvStorage |
//! 
//! ```
//! # 参考:
//! 1. https://www.jianshu.com/p/30970879fbd0
//! 2. https://github.com/zhubaitian/naivecoin/blob/chapter1/README.md

use std::collections::HashMap;

pub mod utils;
use utils::*;

#[derive(Clone, Debug)]
/// Simple block chain for exploration.
pub struct BlockChain{
    utxo: HashMap<String, Block>,
}

#[derive(Clone, Debug)]
pub struct Block{
    

}

#[derive(Debug, Clone)]
pub struct BlockHeader{
    block_size: u32,
    prev_block: HashVal, // 32 Byte
    //version,
    diff: u32, 
    nonce: u32, 
    timestamp: u64,
    merkle_root: HashVal, // 32 Byte
}

#[derive(Clone, Debug)]
pub struct Transcation{
    tx_id: u64,
    input: Vec<TxAddr>,
    output: Vec<TxAddr>,
}


#[cfg(test)]
mod bc_test{
    use super::*;

    #[test]
    fn bc_usage() {
        /*
        let mut chain = SimpleBlockChain::new();

        */
    }   
}