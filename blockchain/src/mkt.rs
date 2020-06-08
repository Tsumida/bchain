use std::fmt;
use std::hash::Hasher;
use std::iter::FromIterator;
use merkletree::hash::{Algorithm, Hashable};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::Sha256;

use digest::{Input, Reset, FixedOutput};

pub struct ExampleAlgorithm(Sha256);

impl ExampleAlgorithm {
    pub fn new() -> ExampleAlgorithm {
        ExampleAlgorithm(Sha256::default())
    }
}

impl Default for ExampleAlgorithm {
    fn default() -> ExampleAlgorithm {
        ExampleAlgorithm::new()
    }
}

impl Hasher for ExampleAlgorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 32]> for ExampleAlgorithm {
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

#[test]
fn test_mkt() {
    let mut h1 = [0u8; 32];
    let mut h2 = [0u8; 32];
    let mut h3 = [0u8; 32];
    h1[0] = 0x11;
    h2[0] = 0x22;
    h3[0] = 0x33;
    // length of v must be 2^p for p >=0 
    let v = vec![h1, h2, h3, h3];
    let t: MerkleTree<[u8; 32], ExampleAlgorithm, VecStore<_>> 
        = MerkleTree::try_from_iter(
            v.into_iter().map(Ok)
        ).unwrap();
    println!("{:?}", t.root());
}