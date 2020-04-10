pub struct BlockChecker{
    pub flags: Vec<u8>,
    pub blocks: Vec<String>,
}

impl BlockChecker{
    pub fn validate(&self) -> bool{
        unimplemented!()
    }
}