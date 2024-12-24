use crate::address::Address;
use crate::block::BlockHash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TransactionHash([u8; 32]);

#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    from: Address,
    to: Address,
    value: u64,
    data: Vec<u8>,
    nonce: u64,
}

impl Transaction {
    pub(crate) fn compute_hash(&self, hash: BlockHash) -> TransactionHash {
        panic!()
    }
}
