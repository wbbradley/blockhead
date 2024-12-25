use crate::address::Address;
use crate::block::BlockHash;
use crate::hash::{Hash, HashBuilder};

#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    pub from_address: Address,
    pub to_address: Address,
    pub value: u64,
    pub data: Vec<u8>,
}

impl Transaction {
    pub(crate) fn compute_hash(&self, hash: BlockHash) -> Hash {
        let mut hasher = HashBuilder::new();
        hasher.update(&hash.0);
        hasher.update(&self.from_address.0);
        hasher.update(&self.to_address.0);
        hasher.update(self.value.to_be_bytes());
        hasher.update(&self.data);
        hasher.finalize()
    }
}
