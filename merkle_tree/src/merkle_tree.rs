//! Merkle tree.
//! References: 
//!     实现细节: https://blog.csdn.net/shangsongwww/article/details/85339243
//! 
use mysha_256::sha_256::SHA256;
pub struct MerkleTree{
    tree: Vec<Vec<String>>,
}

impl MerkleTree{
    pub fn create(mut txs: Vec<String>) -> MerkleTree{
        assert!(txs.len() >= 1);
        txs.sort();
        let mut mt = MerkleTree{
            tree: vec![txs],
        };
        let mut n = mt.tree[0].len();
        while n > 1{
            let mut nodes = Vec::with_capacity((n+1) >> 1);
            for ck in mt.tree.last().unwrap().chunks(2){
                let h = SHA256::new(ck.join("").as_bytes()).cal_sha_256();
                println!("ck = {:?}, h = {}", ck, &h);
                nodes.push(
                    h
                );
            }
            mt.tree.push(nodes);
            n = mt.tree.last().unwrap().len();
        }
        mt      
    }

    // TODO
    pub fn check_existance(&self, h: String) -> bool{
        if let Some(r) = self.tree.last(){
            if let Ok(index) = r.binary_search(&h){
                let mut flags: Vec<u8> = Vec::with_capacity(self.tree.len()); 
                unimplemented!();
            }
            false
        }else{
            false // empty
        }
    }

    pub fn print_tree(&self) {
        for (row_i, row) in self.tree.iter().enumerate(){
            println!("level - {}", row_i);
            for (i, hs) in row.iter().enumerate(){
                println!("i={}, hash={}",i, hs);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test_merkle_tree{
    use super::*;

    #[test]
    fn usage() {
        let case = vec![
            "abc",
            "abcd",
            "abcde",
            "abcdef",
            "abcdefg",
            "abcdefgh",
            "abcdefghi",
        ];

        // the first level hash.
        let hss:Vec<String> = case.into_iter()
            .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
            .collect();

        let _ = MerkleTree::create(hss);
    }

    
}