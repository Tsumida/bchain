use std::hash::Hasher;
use crate::sym_def::{
    HashVal,
    HashAlgorithm,
};

pub trait CoinValue: Clone{
    fn default_value() -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait TxAddr: Clone{
    fn coin_base_addr() -> Self;
}

/// 交易，也就是转账。在Input中，就表
/// 
#[derive(Clone, Debug)]
pub struct Trans<A: TxAddr + AsRef<[u8]>, V: CoinValue >{
    pub addr: A,
    pub val: V,
}

impl<A: TxAddr + AsRef<[u8]>, V: CoinValue > Trans<A, V>{}

#[derive(Clone, Debug)]
pub struct InputTx<A, V>(pub Vec<Trans<A, V>>) 
    where A: TxAddr + AsRef<[u8]>, V: CoinValue ;

#[derive(Clone, Debug)]
pub struct OutputTx<A, V>(pub Vec<Trans<A, V>>) 
    where A: TxAddr + AsRef<[u8]>, V: CoinValue ;

#[derive(Clone, Debug)]
pub struct Transaction<A: TxAddr + AsRef<[u8]>, V: CoinValue >{
    // Assigning before packing into a block, 
    // position of the tx in the tx sequence.
    pub tx_index: u32,

    pub input: InputTx<A, V>,
    pub output: OutputTx<A, V>,
    // sig_script
    // pub_script
}

impl<A: TxAddr + AsRef<[u8]>, V: CoinValue> Transaction<A, V>{
    pub fn coinbase(coin_val: V, recv_addr: A) -> Transaction<A, V>{
        Transaction{
            tx_index: 0,
            input: InputTx(
                vec![Trans{addr: A::coin_base_addr(), val: coin_val.clone()}]
            ),
            output: OutputTx(
                vec![Trans{addr: recv_addr, val: coin_val}]
            ),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        let mut bytes = Vec::new();
        // Bigendian
        // bytes.push(self.tx_index::write_byte::<BigEndian>());
        for trans in self.input.0.iter()
                        .chain(self.output.0.iter())
        {
            bytes.extend(trans.addr.as_ref());
            bytes.extend(trans.val.to_bytes());
        }
        bytes
    }
}

impl<A: TxAddr + AsRef<[u8]>, V: CoinValue> 
    std::convert::Into<HashVal> for Transaction<A, V>
{
    fn into(self) -> HashVal{
        let mut hash = HashAlgorithm::new();
        hash.write(&self.to_bytes());
        hash.take_hash()
    }
}



