
//! Merkle tree.
//! References: 
//!     实现细节: https://www.jianshu.com/p/bfe990be3a21
use mysha_256::sha_256::SHA256;
pub mod bc;
use bc::BlockChecker;

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

    /// Get blocks.
    fn get_blocks(&self, path: &Vec<(usize, usize)>) -> Vec<String>{
        // left node to hashes, right node to rhashes, 
        let top_level = self.tree.len();
        assert!(top_level > 1);
        let mut hashes = Vec::with_capacity(path.len() + 1); // 完全二叉树 n0 = n2 + 1
        let mut rhashes = Vec::new(); // right node's hash

        // left node to hashes, right node to rhashes, 
        for &pos in path.iter().rev(){
            println!("proc {:?}", pos);
            match (MerkleTree::is_tx_node(pos.0), pos){
                (true, (l, i)) => { // tx node => edge.
                    // tx节点的兄弟都要push进去
                    if i % 2 == 0{ 
                        rhashes.push(self.get_slibling_hash(l, i).to_string());  
                        hashes.push(self.tree[l][i].clone());
                    }else{
                        rhashes.push(self.tree[l][i].clone());
                        hashes.push(self.get_slibling_hash(l, i).to_string());
                    }
                },
                (false, (l, i)) => {
                    if i % 2 == 0{ // 左节点需要做计算, 因此右节点push
                        rhashes.push(self.get_slibling_hash(l, i).to_string());  
                    }else{
                        hashes.push(self.get_slibling_hash(l, i).to_string());
                    }
                },
            }
        }
        while let Some(h) = rhashes.pop(){
            hashes.push(h);
        }
        hashes
    }

    fn get_flags(&self, path: &Vec<(usize, usize)>) -> Vec<u8>{
        // flages 是preorder遍历
        // 满二叉树的性质
        let top_level = self.tree.len();
        let mut path = path.clone();
        assert!(top_level > 1);

        let mut flags = Vec::with_capacity(path.len() * 2 + 1);
        let mut st: Vec<(usize, usize)> = Vec::with_capacity(flags.len());

        flags.push(1);
        st.push((top_level-2, 1)); // 第二层开始
        st.push((top_level-2, 0)); 
        //println!("{:?}", path);
        while let Some((l, i)) = st.pop(){
            println!("st={:?}", &st);
            if let Some((pl, pi)) = path.last(){
                if *pl == l && *pi == i{ // match nodes in path.
                    // txnode filter match 
                    // or non-txnode -> need computing
                    flags.push(1); 
                    path.pop();
                    if l > 0{
                        let lens = self.tree.get(l-1).unwrap().len();
                        let q = i << 1;
                        if q + 1 < lens {   st.push((l-1, q+1)); }
                        if q < lens     {   st.push((l-1, q));   }
                    }
                }else{
                    // txnode but filter dismatch 
                    // or use next hash as it's hash.
                    flags.push(0); 
                }
            }else{
                flags.push(0);
            }
            println!("st={:?}, {:?}", &st, &flags);
        }
        //println!("{:?}", &flags);
        flags
    }

    /// Create MerkleTree from non-empty hash sequence using sha-256..
    /// # Example 
    /// ```
    /// use mysha_256::sha_256::SHA256;
    /// use merkle_tree::MerkleTree;
    /// 
    /// let case = vec!["abc", "abcd", "abcde"]
    ///     .into_iter()
    ///     .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
    ///     .collect();
    /// let mt = MerkleTree::create(case);
    /// 
    /// ```
    pub fn create(txs: Vec<String>) -> MerkleTree{
        assert!(txs.len() >= 2);
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
    pub fn gen_block_checker(&self, h: String) -> Option<BlockChecker>{
        let t = &h;
        let mut id = None;
        for (i, tx) in self.tree[0].iter().enumerate(){
            if tx == t{
                id = Some(i);
                break
            }
        }
        if id.is_none(){
            return None
        }
        let mut index = id.unwrap();
        let mut level = 0;
        let top_level = self.tree.len();
        assert!(top_level >= 1);
        let mut path = Vec::with_capacity(top_level);
        
        // get paths.
        while level < top_level - 1{
            path.push((level, index)); // index % 2 == 0 --> left
            index = index >> 1;
            level += 1;
        }
        for p in path.iter(){
            println!("(level={}, index={})", p.0, p.1);
        }
        let blocks = self.get_blocks(&path);
        let flags: Vec<u8> = self.get_flags(&path);

        for (i, e) in blocks.iter().enumerate(){
            println!("{}-{}", i, e);
        }
        println!("flags = {:?}", flags);
        
        Some(
            BlockChecker{
                blocks: blocks,
                flags: flags,
            }
        )
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

    #[inline]
    pub fn is_tx_node(level: usize) -> bool{
        level == 0
    }

    #[inline]
    pub fn is_root(&self, level: usize) -> bool{
        self.tree.len() == level + 1
    }

    pub fn append(&mut self, h: String){
        self.tree[0].push(h.clone());
        self.update_tree(h);
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
            SHA256::new("abcd".as_bytes()).cal_sha_256(),
            mt.get_slibling_hash(0, 0),
        );

        assert_eq!(
            SHA256::new("abc".as_bytes()).cal_sha_256(),
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
    fn test_get_flags() {
        let case = vec![
            "abc", "abcd",
            "abcde", "abcdef",
            "abcdefg", "abcdefgh",
            "abcdefghi",
        ].into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect();
        let mt = MerkleTree::create(case);
        let path = vec![(0, 2), (1, 1), (2, 0)];
        assert_eq!(
            vec![1, 1, 0, 1, 1, 0, 0],
            mt.get_flags(&path),
        );

        let path = vec![(0, 0), (1, 0), (2, 0)];
        assert_eq!(
            vec![1, 1, 1, 1, 0, 0, 0],
            mt.get_flags(&path),
        );
    }

    #[test]
    fn test_level_len() {
        // level0 -> 7, level1 -> 4, level2->2, level3->1
        let case2 = vec![
            "abc",
            "abcd",
            "abcde",
            "abcdef",
            "abcdefg",
            "abcdefgh",
            "abcdefghi",
        ].into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect(); 

        let mt2 = MerkleTree::create(case2);
        assert_eq!(
            4, 
            mt2.get_height()
        );
        assert_eq!(
            "e239682fd4c8efb7a008f355d8c9858a2b458fee1e4ba1bc86dd032fe745d204",
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

    #[test]
    fn test_gen_block_checker() {
        // the first level hash.
        let case = vec![
            "abc",
            "abcd",
            "abcde",
            "abcdef",
            "abcdefg",
            "abcdefgh",
            "abcdefghi",
        ].into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect();
        let mt = MerkleTree::create(case);
        let _ = mt.gen_block_checker(SHA256::new("abcde".as_bytes()).cal_sha_256());

    }
    
    #[test]
    fn test_usage() {
        let case = vec![
            "abc",
            "abcd",
            "abcde",
            "abcdef",
            "abcdefg",
            "abcdefgh",
            "abcdefghi",
        ].into_iter()
        .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
        .collect();
        let mt = MerkleTree::create(case);

        mt.print_tree();
    }
}

pub struct SortedMerkleTree{
    tree: Vec<Vec<String>>,
}

impl SortedMerkleTree{
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

    /// Get blocks.
    fn get_blocks(&self, path: &Vec<(usize, usize)>) -> Vec<String>{
        // left node to hashes, right node to rhashes, 
        let top_level = self.tree.len();
        assert!(top_level > 1);
        let mut hashes = Vec::with_capacity(path.len() + 1); // 完全二叉树 n0 = n2 + 1
        let mut rhashes = Vec::new(); // right node's hash

        // left node to hashes, right node to rhashes, 
        for &pos in path.iter().rev(){
            println!("proc {:?}", pos);
            match (MerkleTree::is_tx_node(pos.0), pos){
                (true, (l, i)) => { // tx node => edge.
                    // tx节点的兄弟都要push进去
                    if i % 2 == 0{ 
                        rhashes.push(self.get_slibling_hash(l, i).to_string());  
                        hashes.push(self.tree[l][i].clone());
                    }else{
                        rhashes.push(self.tree[l][i].clone());
                        hashes.push(self.get_slibling_hash(l, i).to_string());
                    }
                },
                (false, (l, i)) => {
                    if i % 2 == 0{ // 左节点需要做计算, 因此右节点push
                        rhashes.push(self.get_slibling_hash(l, i).to_string());  
                    }else{
                        hashes.push(self.get_slibling_hash(l, i).to_string());
                    }
                },
            }
        }
        while let Some(h) = rhashes.pop(){
            hashes.push(h);
        }
        hashes
    }

    fn get_flags(&self, path: &Vec<(usize, usize)>) -> Vec<u8>{
        // flages 是preorder遍历
        // 满二叉树的性质
        let top_level = self.tree.len();
        let mut path = path.clone();
        assert!(top_level > 1);

        let mut flags = Vec::with_capacity(path.len() * 2 + 1);
        let mut st: Vec<(usize, usize)> = Vec::with_capacity(flags.len());

        flags.push(1);
        st.push((top_level-2, 1)); // 第二层开始
        st.push((top_level-2, 0)); 
        //println!("{:?}", path);
        while let Some((l, i)) = st.pop(){
            println!("st={:?}", &st);
            if let Some((pl, pi)) = path.last(){
                if *pl == l && *pi == i{ // match nodes in path.
                    // txnode filter match 
                    // or non-txnode -> need computing
                    flags.push(1); 
                    path.pop();
                    if l > 0{
                        let lens = self.tree.get(l-1).unwrap().len();
                        let q = i << 1;
                        if q + 1 < lens {   st.push((l-1, q+1)); }
                        if q < lens     {   st.push((l-1, q));   }
                    }
                }else{
                    // txnode but filter dismatch 
                    // or use next hash as it's hash.
                    flags.push(0); 
                }
            }else{
                flags.push(0);
            }
            println!("st={:?}, {:?}", &st, &flags);
        }
        //println!("{:?}", &flags);
        flags
    }

    /// Create a SortedMerkleTree from non-empty hash sequence using sha-256. 
    /// Hashes of Transactions will be sorted before further construction.
    /// # Example 
    /// ```
    /// use mysha_256::sha_256::SHA256;
    /// use merkle_tree::SortedMerkleTree;
    /// 
    /// let case = vec!["abc", "abcd", "abcde"]
    ///     .into_iter()
    ///     .map(|b| SHA256::new(b.as_bytes()).cal_sha_256())
    ///     .collect();
    /// let mt = SortedMerkleTree::create(case);
    /// 
    /// ```
    pub fn create(mut txs: Vec<String>) -> SortedMerkleTree{
        assert!(txs.len() >= 2);
        // if txs is sorted and then we can perform Inclusive Verifacation in O(logN).
        // But it will slow append(), remove(), update() due to re-sort and O(N) re-hash.
        txs.sort();
        let mut smt = SortedMerkleTree{
            tree: vec![txs],
        };
        smt.build();
        assert!(smt.tree.len() >= 1);
        smt      
    }

    // TODO
    pub fn gen_block_checker(&self, h: String) -> Option<BlockChecker>{
        let t = &h;
        let mut id = None;
        for (i, tx) in self.tree[0].iter().enumerate(){
            if tx == t{
                id = Some(i);
                break
            }
        }
        if id.is_none(){
            return None
        }
        let mut index = id.unwrap();
        let mut level = 0;
        let top_level = self.tree.len();
        assert!(top_level >= 1);
        let mut path = Vec::with_capacity(top_level);
        
        // get paths.
        while level < top_level - 1{
            path.push((level, index)); // index % 2 == 0 --> left
            index = index >> 1;
            level += 1;
        }
        for p in path.iter(){
            println!("(level={}, index={})", p.0, p.1);
        }
        let blocks = self.get_blocks(&path);
        let flags: Vec<u8> = self.get_flags(&path);

        for (i, e) in blocks.iter().enumerate(){
            println!("{}-{}", i, e);
        }
        println!("flags = {:?}", flags);
        
        Some(
            BlockChecker{
                blocks: blocks,
                flags: flags,
            }
        )
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
    
    #[inline]
    pub fn is_tx_node(level: usize) -> bool{
        level == 0
    }

    #[inline]
    pub fn is_root(&self, level: usize) -> bool{
        self.tree.len() == level + 1
    }

    pub fn get_height(&self) -> usize{
        self.tree.len()
    }

    /// Get hash of  merkle root.
    pub fn get_root_hash(&self) -> Option<String>{
        Some(self.tree.last().unwrap()[0].clone())
    }
}