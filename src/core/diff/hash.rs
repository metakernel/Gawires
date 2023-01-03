use std::io::Read;

use blake2::{Blake2b512, Digest};

#[derive(PartialEq, Clone, Debug)]
pub struct DiffHash
{
    pub value: Option<Vec<u8>>,
}

pub struct DiffHasher
{
    hasher: Blake2b512,
    pub digest: DiffHash,
}

impl DiffHasher
{
    fn new() -> Self
    {
        Self {
            hasher: Blake2b512::new(),
            digest: DiffHash { value: None },
        }
    }

    fn update(&mut self, data: &[u8])
    {
        self.hasher.update(data);
    }

    fn finalize(&mut self)
    {
        let hash = self.hasher.clone().finalize();
        self.digest.value = Some(hash.to_vec());
    }

    pub fn hash_file(path: &str) -> DiffHash
    {
        let mut hasher = DiffHasher::new();
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = [0; 1024];
        loop {
            let count = file.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        // Store the hash value
        hasher.finalize();

        // Return the hash value
        hasher.digest
    }

    
}

impl DiffHash
{
    pub fn new() -> Self
    {
        Self { value: None }
    }

    pub fn get_value(&self) -> Option<Vec<u8>>
    {
        self.value.clone()
    }
}