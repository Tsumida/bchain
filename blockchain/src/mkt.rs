use std::hash::Hasher;
use merkletree::hash::{Algorithm, Hashable};
use merkletree::merkle::{
    //MerkleTree,
    Element,
};
//use merkletree::store::VecStore;
use sha2::Sha256;

use digest::{Input, Reset, FixedOutput};


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct HashVal(pub [u8; 32]);

impl Default for HashVal{
    fn default() -> Self{
        HashVal([0; 32])
    }
}

impl AsRef<[u8]> for HashVal{
    fn as_ref(&self) -> &[u8]{
        &self.0
    }
}

impl Element for HashVal{
    fn byte_len() -> usize{
        32
    }

    fn from_slice(bytes: &[u8]) -> Self{
        if bytes.len() != 32{
            panic!("Non-appropriate size.")
        }
        let mut h = [0; 32];
        h.copy_from_slice(bytes);
        HashVal(h)
    }

    fn copy_to_slice(&self, bytes: &mut [u8]){
        if bytes.len() < 32{
            panic!("Non-appropriate size.")
        }
        for (i, e) in bytes.iter_mut().enumerate(){
            *e = self.0[i];
        }
    }
}


#[derive(Clone, Debug)]
pub struct HashAlgorithm(Sha256);

impl HashAlgorithm {
    pub fn new() -> HashAlgorithm {
        HashAlgorithm(Sha256::default())
    }
}

impl Default for HashAlgorithm {
    fn default() -> HashAlgorithm {
        HashAlgorithm::new()
    }
}

impl Hasher for HashAlgorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<HashVal> for HashAlgorithm {
    #[inline]
    fn hash(&mut self) -> HashVal {
        let mut h = [0u8; 32];
        for (i, e) in self.0.clone().fixed_result().iter().enumerate(){
            h[i] = *e;
        }
        HashVal(h)
    }
    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}
