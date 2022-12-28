use blake2::{Blake2b512, Digest};

#[derive(PartialEq, Clone, Debug)]
pub struct DiffHash
{
    pub value: Vec<u8>,
}

pub struct DiffHasher
{
    hasher: Blake2b512,
    pub digest: DiffHash,
}