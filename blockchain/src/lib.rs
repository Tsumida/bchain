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

use std::collections::LinkedList;
use std::iter::{FromIterator, Iterator};
use std::io::{Read, Write};
// use std::marker::PhantomData;
use std::time::Instant; 

use merkletree::merkle::{Element, MerkleTree};
use digest::{Input, FixedOutput};
use sha2::Sha256;
use byteorder::{WriteBytesExt, BigEndian};


mod block;
mod transaction;
mod sym_def;

use sym_def::{
    HashVal,
    HashAlgorithm,
};
use block::*;
use transaction::*;

#[derive(Clone, Debug)]
/// Simple block chain for exploration.
pub struct BlockChain{
    chain: LinkedList<Block>, 
}

impl BlockChain{
    pub fn new() -> BlockChain{
        let mut c = LinkedList::new();
        c.push_back(Block::genesis_block(0));
        BlockChain{
            chain: c, // ts
        }
    }

    pub fn push(&mut self, txs: impl IntoIterator<Item=SimpleTx>){

        // let b = Block::pack(ts: u64, txs: impl Iterator<Item=Transaction<A, V>>)
        // self.chain.push_back(b);
    }
}

type SimpleHash = HashVal;
type SimpleTx = Transaction<String, SimpleValue>;
type SimpleChain = BlockChain;

type SimpleValue = f64;

impl CoinValue for SimpleValue{
    fn default_value() -> Self{
        SimpleValue::from(100)
    }

    fn to_bytes(&self) -> Vec<u8>{
        self.to_be_bytes().to_vec()
    }
}

impl SimpleTx{
    /// Tx -> SHA256
    pub fn to_simple_hash(&self) -> SimpleHash{
        let mut sha = sha2::Sha256::default();
        let mut h: SimpleHash = HashVal([0; 32]);

        sha.input(self.to_bytes());

        for (i, e) in sha.fixed_result().iter().enumerate(){
            *h.0.get_mut(i).unwrap() = *e;
        }
        h
    }
}

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

    // create 3 txs: 1 coinbase, 2 transactions.
    let mut txs = vec![
        // 100 coin -> A
        Transaction::coinbase(100.0, addr_a.clone()),
        // A ---> 100 ---> 95 --> A
        //         | ----> 5  --> B
        Transaction{
            tx_index: 1,
            input:InputTx(vec![
                Trans{addr: addr_a.clone(), val: SimpleValue::from(100.0)},
            ]),
            output:OutputTx(vec![
                Trans{addr: addr_b.clone(), val: SimpleValue::from(95.0)}, 
                Trans{addr: addr_a.clone(), val: SimpleValue::from(5.0)},
            ]),
        }, 
        // B ---> 2 ----> C
        Transaction{
            tx_index: 2,
            input:InputTx(vec![
                Trans{addr: addr_b.clone(), val: SimpleValue::from(2.0)},
            ]),
            output:OutputTx(vec![
                Trans{addr: addr_c.clone(), val: SimpleValue::from(2.0)}, 
            ]),
        }
        // final --- : A: 95.0, B: 3.0, C: 2.0
    ];

    // merkletree takes exact 2^k block as input, where k >= 1
    let n = txs.len();
    assert!(n > 0); 
    let pow2 = n.next_power_of_two();
    if pow2 != n{
        let delta = pow2 - n;
        txs.extend(
            std::iter::repeat(
                txs.last().unwrap().clone())
            .take(delta)
            .collect::<Vec<Transaction<_, _>>>()
        );
    }

    // tx -> hash
    let block = Block::pack(10000, txs);

    // mkt root -> Blockchain
    let root = block.mkt_root();
    eprintln!("merkle root: {:?}", root);

    // TODO: take timestamp.
    //let block = Block::pack(ts: u64, txs: impl Iterator<Item=Transaction<A, V>>);

}   

#[test]
fn test_adding(){
    let p: u8 = 0b01100110;
    assert_eq!( 0b11001100u8, p.wrapping_shl(1));
    assert_eq!( 0b10011000u8, p.wrapping_shl(2));

    assert_eq!(0b10011001u8, p.rotate_left(2));
}