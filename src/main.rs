#![allow(dead_code)]
//!
//! This interface covers the main categories of blockchain interactions:
//!
//! 1. Block queries: Fetching blocks by hash/number and latest block
//! 2. Transaction operations: Querying, sending, and getting receipts
//! 3. Account operations: Balance and nonce queries
//! 4. Contract interactions: Calls and gas estimation
//! 5. Chain information: Chain ID, sync status, gas price
//!
//! The trait uses async/await for all operations since blockchain RPCs are typically network
//! calls. The mock implementation provides a basic example of how these could be implemented.
//!
use crate::error::Result;
use std::{collections::HashMap, path::Path};
mod error;

type BlockHash = String;
type Address = String;
type TransactionHash = String;

#[derive(Debug, Clone)]
struct Block {
    hash: BlockHash,
    parent_hash: BlockHash,
    number: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
struct Transaction {
    hash: TransactionHash,
    from: Address,
    to: Address,
    value: u64,
    data: Vec<u8>,
    nonce: u64,
}

#[derive(Debug)]
struct TransactionReceipt {
    transaction_hash: TransactionHash,
    block_hash: BlockHash,
    status: bool,
    gas_used: u64,
    logs: Vec<Log>,
}

#[derive(Debug)]
struct Log {
    address: Address,
    topics: Vec<String>,
    data: Vec<u8>,
}

#[async_trait::async_trait]
trait Blockchain {
    // Block related
    async fn get_block_by_hash(&self, hash: BlockHash) -> Option<Block>;
    async fn get_block_by_number(&self, number: u64) -> Option<Block>;
    async fn get_latest_block(&self) -> Block;

    // Transaction related
    async fn get_transaction(&self, hash: TransactionHash) -> Option<Transaction>;
    async fn get_transaction_receipt(&self, hash: TransactionHash) -> Option<TransactionReceipt>;
    async fn send_transaction(&self, transaction: Transaction) -> TransactionHash;

    // Account related
    async fn get_balance(&self, address: Address) -> u64;
    async fn get_nonce(&self, address: Address) -> u64;

    // Contract related
    async fn call(&self, to: Address, data: Vec<u8>) -> Vec<u8>;
    async fn estimate_gas(&self, to: Address, data: Vec<u8>) -> u64;

    // Chain related
    async fn chain_id(&self) -> u64;
    async fn syncing(&self) -> bool;
    async fn gas_price(&self) -> u64;
}

struct Blockhead {
    connection: sqlite::ConnectionThreadSafe,

    blocks: HashMap<BlockHash, Block>,
    transactions: HashMap<TransactionHash, Transaction>,
    balances: HashMap<Address, u64>,
}

impl Blockhead {
    fn new<T: AsRef<Path>>(db_filename: T) -> Result<Self> {
        let connection = sqlite::Connection::open_thread_safe(db_filename)?;

        let query = "
            CREATE TABLE blocks (id UUID, parent_id, payload INTEGER);
        ";
        assert!(connection.execute(query).is_ok());
        Ok(Self {
            connection,
            blocks: Default::default(),
            transactions: Default::default(),
            balances: Default::default(),
        })
    }
}

#[async_trait::async_trait]
impl Blockchain for Blockhead {
    async fn get_block_by_hash(&self, hash: BlockHash) -> Option<Block> {
        self.blocks.get(&hash).cloned()
    }

    async fn get_block_by_number(&self, number: u64) -> Option<Block> {
        self.blocks
            .values()
            .find(|block| block.number == number)
            .cloned()
    }

    async fn get_latest_block(&self) -> Block {
        self.blocks
            .values()
            .max_by_key(|block| block.number)
            .cloned()
            .unwrap()
    }

    async fn get_transaction(&self, hash: TransactionHash) -> Option<Transaction> {
        self.transactions.get(&hash).cloned()
    }

    async fn get_transaction_receipt(&self, _hash: TransactionHash) -> Option<TransactionReceipt> {
        // Implementation omitted for brevity
        None
    }

    async fn send_transaction(&self, _transaction: Transaction) -> TransactionHash {
        "mock_tx_hash".to_string()
    }

    async fn get_balance(&self, address: Address) -> u64 {
        *self.balances.get(&address).unwrap_or(&0)
    }

    async fn get_nonce(&self, _address: Address) -> u64 {
        0
    }

    async fn call(&self, _to: Address, _data: Vec<u8>) -> Vec<u8> {
        vec![]
    }

    async fn estimate_gas(&self, _to: Address, _data: Vec<u8>) -> u64 {
        21000
    }

    async fn chain_id(&self) -> u64 {
        1
    }

    async fn syncing(&self) -> bool {
        false
    }

    async fn gas_price(&self) -> u64 {
        20_000_000_000
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Blockhead::new(":memory:")?;
    let balance = client.get_balance("0x123...".to_string()).await;
    let gas_price = client.gas_price().await;
    println!("Balance: {}, Gas Price: {}", balance, gas_price);
    Ok(())
}

#[test]
fn test_sqlite_mem() {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
    ";
    assert!(connection.execute(query).is_ok());
}
