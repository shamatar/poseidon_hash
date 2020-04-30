use crate::tiny_keccak::Keccak;
use crate::blake2_rfc::blake2s::Blake2s;
pub trait GroupHasher {
    fn new(personalization: &[u8]) -> Self;
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

pub struct BlakeHasher {
    h: Blake2s
}

impl GroupHasher for BlakeHasher {
    fn new(personalization: &[u8]) -> Self {
        let h = Blake2s::with_params(32, &[], &[], personalization);

        Self {
            h: h
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.h.update(data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        let new_h = Blake2s::with_params(32, &[], &[], &[]);
        let h = std::mem::replace(&mut self.h, new_h);

        let result = h.finalize();

        result.as_ref().to_vec().clone()
    }
}

pub struct Keccak256Hasher {
    h: Keccak
}

impl GroupHasher for Keccak256Hasher {
    fn new(personalization: &[u8]) -> Self {
        let mut h = Keccak::new_keccak256();
        h.update(personalization);

        Self {
            h: h
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.h.update(data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        let new_h = Keccak::new_keccak256();
        let h = std::mem::replace(&mut self.h, new_h);

        let mut res: [u8; 32] = [0; 32];
        h.finalize(&mut res);

        res[..].to_vec()
    }
}