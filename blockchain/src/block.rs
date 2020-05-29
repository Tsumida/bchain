//!
//! 简单区块链结构:
//! ```
//! 
//! ```



#[derive(Clone, Debug)]
/// Simple block chain for exploration.
pub struct BlockChain{

}

#[derive(Clone, Debug)]
pub struct Block{
    tx_cnt: u32,                        // 4B
    txs: Vec<Transcation>,
}

#[derive(Clone, Debug)]
pub struct Transcation{
    tx_id: u64,
}


#[cfg(test)]
mod sbc_test{
    use super::*;

    #[test]
    fn sbc_usage() {
        /*
        let mut chain = SimpleBlockChain::new();

        */
    }   
}