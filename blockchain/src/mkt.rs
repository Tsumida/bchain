use std::fmt;
use std::hash::Hasher;
use std::iter::FromIterator;
use merkletree::hash::{Algorithm, Hashable};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::Sha256;

use digest::{Input, Reset, FixedOutput};

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

impl Algorithm<[u8; 32]> for HashAlgorithm {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let mut h = [0u8; 32];
        for (i, e) in self.0.clone().fixed_result().iter().enumerate(){
            h[i] = *e;
        }
        h
    }
    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}
