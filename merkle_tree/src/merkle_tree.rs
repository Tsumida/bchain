//! Merkle tree.
//! References: 
//!     实现细节: https://blog.csdn.net/shangsongwww/article/details/85339243
//! 
use mysha_256::sha_256::SHA256;
pub struct MerkleTree{
    tree: Vec<Vec<String>>,
}

static NO_SLIBLING: &'static str = "";

impl MerkleTree{
    fn build(&mut self){
        let mut n = self.tree[0].len();
        while n > 1{
            let mut nodes = Vec::with_capacity((n+1) >> 1);
            for ck in self.tree.last().unwrap().chunks(2){
                let h = SHA256::new(ck.join("").as_bytes()).cal_sha_256();
                //println!("ck = {:?}, h = {}", ck, &h);
                nodes.push(h);
            }
            self.tree.push(nodes);
            n = self.tree.last().unwrap().len();
        }
        assert_eq!(1, self.tree.last().unwrap().len());
    }

    fn get_slibling_hash(&self, row: usize, index: usize) -> &str{
        // index row is ok
        let n = self.tree[row].len();
        if index >= n{
            NO_SLIBLING
        }else{
            if index % 2 == 0{ // left, 0 <= index < len()
                if index + 1 < n{ 
                    &self.tree[row][index+1]
                }else{
                    NO_SLIBLING
                }
            }else{ // right, index >= 1
                &self.tree[row][index-1]
            }
        }
    }

    fn update_tree(&mut self, h: String){
        let mut index = self.tree[0].len() - 1;
        let mut current_hash = h;
        let top_level = self.tree.len() - 1;
        let mut level_index = 0;
        while level_index < top_level{
            let p_index = index >> 1;

            let slib_hash = self.get_slibling_hash(level_index, index);
            if index % 2 == 0 { // left
                current_hash = SHA256::new((current_hash + slib_hash).as_bytes()).cal_sha_256();
            }else{
                current_hash = SHA256::new((slib_hash.to_string() + &current_hash).as_bytes()).cal_sha_256();
            }

            if p_index >= self.tree[level_index+1].len(){
                self.tree[level_index+1].push(current_hash.clone());
            }else{
                self.tree[level_index+1][p_index] = current_hash.clone();
            }
            index = self.tree[level_index+1].len() - 1;
            level_index += 1;
        }
        // <= 2, because mt append only one ele each time.
        let top_len = self.tree.last().unwrap().len();
        if top_len > 1{ 
            assert_eq!(2, top_len);
            self.tree.push(
                vec![
                    SHA256::new(self.tree.last()
                    .unwrap()
                    .join("")
                    .as_bytes())
                    .cal_sha_256()
                ]
            );
        }
    }

    /// Create MerkleTree from non-empty hash sequence using sha-256..
    /// # Example 
    /// ```
    /// let case = vec!["abc", "abcd", "abcde"].into_iter()
    ///     .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
    ///     .collect();
    /// let mt = MerkleTree::create(hss);
    /// ```
    pub fn create(txs: Vec<String>) -> MerkleTree{
        assert!(txs.len() >= 1);
        // if txs is sorted and then we can perform Inclusive Verifacation in O(logN).
        // But it will slow append(), remove(), update() due to re-sort and O(N) re-hash.
        // txs.sort();
        let mut mt = MerkleTree{
            tree: vec![txs],
        };
        mt.build();
        assert!(mt.tree.len() >= 1);
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

    pub fn get_height(&self) -> usize{
        self.tree.len()
    }

    pub fn append(&mut self, h: String){
        self.tree[0].push(h.clone());
        self.update_tree(h);
    }

    pub fn update(&mut self, oh: String, nh: String){
        unimplemented!();
    }

    pub fn remove(&mut self, h: String){
        unimplemented!();

    }

    /// Get hash of  merkle root.
    pub fn get_root_hash(&self) -> Option<String>{
        Some(self.tree.last().unwrap()[0].clone())
    }
}

#[cfg(test)]
mod test_merkle_tree{
    use super::*;

    #[test]
    fn test_get_slib_hash() {
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
        let mt = MerkleTree::create(hss);

        assert_eq!(
            "36bbe50ed96841d10443bcb670d6554f0a34b761be67ec9c4a8ad2c0c44ca42c",
            mt.get_slibling_hash(0, 0),
        );

        assert_eq!(
            "19cc02f26df43cc571bc9ed7b0c4d29224a3ec229529221725ef76d021c8326f",
            mt.get_slibling_hash(0, 1),
        );

        assert_eq!(
            NO_SLIBLING,
            mt.get_slibling_hash(0, 6),
        );

        assert_eq!(
            NO_SLIBLING,
            mt.get_slibling_hash(0, 1024),
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_get_slib_hash(){
        MerkleTree::create(
            vec!["36bbe50ed96841d10443bcb670d6554f0a34b761be67ec9c4a8ad2c0c44ca42c".to_string()]
        ).get_slibling_hash(1024, 1);
    }

    #[test]
    fn test_level_len() {
        let case1 = vec!["abc"].into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect();
        let mt1 = MerkleTree::create(
            case1
        );
        assert_eq!(1, mt1.get_height());
        assert_eq!(
            SHA256::new("abc".as_bytes()).cal_sha_256(),
            mt1.get_root_hash().unwrap()
        );

        // level0 -> 7, level1 -> 4, level2->2, level3->1
        let case2 = vec![
            "abc",
            "abcd",
            "abcde",
            "abcdef",
            "abcdefg",
            "abcdefgh",
            "abcdefghi",
        ]
        .into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect(); 

        let mut mt2 = MerkleTree::create(case2);
        assert_eq!(
            4, 
            mt2.get_height()
        );
        assert_eq!(
            "7c7902a90b4c6cb946f50ce19d04784aa0d1de001ff138b0644db90298cd292d",
            mt2.get_root_hash().unwrap()
        );
    }

    #[test]
    fn test_append() {
        let case2 = vec!["abc", "abcd", "abcde",]
        .into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect();

        let mut mt = MerkleTree::create(case2);
        assert_eq!(3, mt.get_height());

        mt.append(SHA256::new("abcdef".as_bytes()).cal_sha_256());
        assert_eq!(3, mt.get_height());

        mt.append(SHA256::new("abcdefg".as_bytes()).cal_sha_256());
        assert_eq!(4, mt.get_height());

        assert_eq!(
            mt.get_root_hash(),
            MerkleTree::create(
                vec!["abc", "abcd", "abcde", "abcdef", "abcdefg"]
                .into_iter()
                .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
                .collect()
            ).get_root_hash()
        )

    }
    
}