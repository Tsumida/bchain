//! # 概述
//! 模仿BTC的架构实现。
//! 
//! 首先BTC本质上是一个分布式状态机，状态即UTXO的集合：
//! 
//! ```
//!     Net：               P2P, Gossip
//!                             |
//!                         BlockChecker
//!                              |
//!                            UXTO
//!     Mem:               | MemPool   |------- Mining(Pow, Diff, )
//!                        |     |     |
//!     Stablization :     | KvStorage |
//! ```
//! # 运行流程
//! 1. 创建交易，验证目的节点地址
//! 2. 对交易进行签名
//! 3. 将该交易广播到全网节点
//! 4. 全节点接受交易并且验证有效性。需要全网多数节点验证接受。
//! 5. 交易暂存在本节点内存池，判断是否孤立交易（？）
//! 6. 交易打包到区块中
//! 7. 这一轮共识接受，获胜节点将区块追加到主链上
//! 8. 被更多后序区块确认。
//! 
//! 还有很多细节、容错要考虑，比如：
//! 1. 非最长链，怎么办？
//! 2. 落后区块如何连接
//! 
//! # 参考:
//! 1. https://www.jianshu.com/p/30970879fbd0
//! 2. https://github.com/zhubaitian/naivecoin/blob/chapter1/README.md
//! 
//! # V1
//! 没有签名的区块链

use std::collections::HashMap;
use std::iter::{FromIterator, Iterator};
use std::io::{Read, Write};
use std::marker::PhantomData;

use merkletree::merkle::{Element, MerkleTree};
use mkt::HashAlgorithm;
use digest::{Input, FixedOutput};
use sha2::Sha256;
use byteorder::{ WriteBytesExt, BigEndian};

mod utils;
mod block;
mod transaction;
mod mkt;
//use mkt::*;
use utils::*;
use block::*;
use transaction::*;


#[derive(Clone, Debug)]
/// Simple block chain for exploration.
pub struct BlockChain{
    chain: Vec<Block>, 
}

impl BlockChain{
    pub fn new() -> BlockChain{
        let genesis_block = Block::genesis_block(0);
        BlockChain{
            chain: vec![genesis_block], // ts
        }
    }

    pub fn push(&mut self, txs: impl IntoIterator<Item=SimpleTx>){

    }

}

type SimpleHash = [u8; 32];
type SimpleTx = Transaction<String, SimpleValue>;
type SimpleChain = BlockChain;

#[derive(Clone, Debug)]
pub struct SimpleValue{
    pub val: u64, // v64
}

impl CoinValue for SimpleValue{
    fn default_value() -> Self{
        SimpleValue::from(100)
    }

    fn to_bytes(&self) -> Vec<u8>{
        let mut v = Vec::with_capacity(std::mem::size_of_val(&self.val));
        v.write_u64::<BigEndian>(self.val).unwrap();
        v
    }
}

impl From<u64> for SimpleValue{
    fn from(v: u64) -> SimpleValue{
        SimpleValue{
            val: v
        }
    }
}

impl SimpleTx{
    /// Tx -> SHA256
    pub fn to_simple_hash(&self) -> SimpleHash{
        let mut sha = sha2::Sha256::default();
        let mut h: SimpleHash = [0; 32];

        sha.input(self.to_bytes());

        for (i, e) in sha.fixed_result().iter().enumerate(){
            *h.get_mut(i).unwrap() = *e;
        }
        h
    }
}

pub trait HashVal: Element{}
impl HashVal for SimpleHash{}

impl TxAddr for String{
    fn coin_base_addr() -> Self{
        "".to_string()
    }
}

#[test]
fn bc_usage() {
    let mut chain: SimpleChain = BlockChain::new();
    let txs:Vec<SimpleTx> = Vec::new();

  
    let addr_a = "Alice".to_string();
    let addr_b = "Bob".to_string();
    let addr_c = "Carona".to_string();

    let txs = vec![
        Transaction{
            input:InputTx(vec![
                Trans{addr: addr_a.clone(), val: SimpleValue::from(10)},
            ]),
            output:OutputTx(vec![
                Trans{addr: addr_b.clone(), val: SimpleValue::from(5)}, 
                Trans{addr: addr_a.clone(), val: SimpleValue::from(5)},
            ]),
        }, 
        Transaction{
            input:InputTx(vec![
                Trans{addr: addr_b.clone(), val: SimpleValue::from(2)},
            ]),
            output:OutputTx(vec![
                Trans{addr: addr_c.clone(), val: SimpleValue::from(2)}, 
            ]),
        }
    ];
    // tx -> hash
    let tx_hash:Vec<SimpleHash> = txs.iter().map(|tx|  {
        let mut s = Sha256::default();
        let mut h = [0; 32];
        s.input(tx.to_bytes()); 
        for (i, e) in s.fixed_result().into_iter().enumerate(){
            h[i] = e;
        }
        h
    }).collect();

    let mkt: MerkleTree<SimpleHash, HashAlgorithm, merkletree::store::VecStore<SimpleHash>>
        = MerkleTree::try_from_iter(tx_hash.into_iter().map(Ok)).unwrap();

    // mkt root -> Blockchain
    let root = mkt.root();
    eprintln!("merkle root: {:?}", root);

}   