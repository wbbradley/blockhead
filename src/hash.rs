use blake2::{Blake2s256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Hash(pub [u8; 32]);

impl From<&str> for Hash {
    fn from(s: &str) -> Self {
        let mut hasher = HashBuilder::new();
        hasher.update(s.as_bytes());
        hasher.finalize()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}
pub(crate) struct HashBuilder {
    hasher: Blake2s256,
}

// HashBuilder is currently built on Blake2s256, which is a 256-bit hash function.
impl HashBuilder {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            hasher: Blake2s256::new(),
        }
    }
    #[inline]
    pub(crate) fn update<T: std::convert::AsRef<[u8]>>(&mut self, data: T) {
        self.hasher.update(data);
    }
    #[inline]
    pub(crate) fn finalize(self) -> Hash {
        Hash(self.hasher.finalize().into())
    }
}
