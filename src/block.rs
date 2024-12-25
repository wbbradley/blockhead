use crate::hash::Hash;
use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub hash: Hash,
    pub parent_hash: Hash,
    pub number: u64,
    pub timestamp: u64,
    pub transactions: Vec<(Hash, Transaction)>,
}
