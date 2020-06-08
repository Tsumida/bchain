
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
    pub input: InputTx<A, V>,
    pub output: OutputTx<A, V>,
    // sig_script
    // pub_script
}

impl<A: TxAddr + AsRef<[u8]>, V: CoinValue > Transaction<A, V>{
    pub fn coinbase(coin_val: V, recv_addr: A) -> Transaction<A, V>{
        Transaction{
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


        for trans in self.input.0.iter()
                        .chain(self.output.0.iter())
        {
            bytes.extend(trans.addr.as_ref());
            bytes.extend(trans.val.to_bytes());
        }
        bytes
    }
}




