/// An address in the blockhead blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Address(pub [u8; 32]);
