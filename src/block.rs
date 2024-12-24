use crate::transaction::{Transaction, TransactionHash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BlockHash([u8; 32]);

impl std::fmt::Display for BlockHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Block {
    hash: BlockHash,
    parent_hash: BlockHash,
    number: u64,
    timestamp: u64,
    transactions: Vec<(TransactionHash, Transaction)>,
}
