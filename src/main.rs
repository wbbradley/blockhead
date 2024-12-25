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
use crate::address::Address;
use crate::block::Block;
use crate::error::Result;
use crate::hash::Hash;
use crate::transaction::Transaction;
use std::{collections::HashMap, path::Path};

mod address;
mod block;
mod error;
mod hash;
mod transaction;

#[derive(Debug)]
struct TransactionReceipt {
    transaction_hash: Hash,
    block_hash: Hash,
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
    async fn get_block_by_hash(&self, hash: Hash) -> Result<Option<Block>>;
    async fn get_block_by_number(&self, number: u64) -> Option<Block>;
    async fn get_latest_block(&self) -> Block;

    // Transaction related
    async fn get_transaction(&self, hash: Hash) -> Option<Transaction>;
    async fn get_transaction_receipt(&self, hash: Hash) -> Option<TransactionReceipt>;
    async fn send_transaction(&self, transaction: Transaction) -> Hash;

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

    blocks: HashMap<Hash, Block>,
    transactions: HashMap<Hash, Transaction>,
    balances: HashMap<Address, u64>,
}

impl Blockhead {
    fn new<T: AsRef<Path>>(db_filename: T) -> Result<Self> {
        let connection = sqlite::Connection::open_thread_safe(db_filename)?;

        let query = "
            CREATE TABLE block (
                hash TEXT,
                parent_hash TEXT,
                number INTEGER,
                timestamp_nanos INTEGER
            );
            CREATE TABLE transactions (
                hash TEXT,
                block_hash TEXT,
                from_address TEXT,
                to_address TEXT,
                value INTEGER,
                data BLOB,
                nonce INTEGER
            );
        ";
        connection.execute(query)?;
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
    async fn get_block_by_hash(&self, hash: Hash) -> Result<Option<Block>> {
        let query = "SELECT * FROM block WHERE hash = ? LIMIT 1";
        let hash_string: String = hash.to_string();
        for row in self
            .connection
            .prepare(query)?
            .into_iter()
            .bind((1, hash_string.as_str()))
            .unwrap()
        {
            let row = row?;
            println!("name = {}", row.read::<&str, _>("name"));
            println!("age = {}", row.read::<i64, _>("age"));
        }
        Ok(self.blocks.get(&hash).cloned())
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

    async fn get_transaction(&self, hash: Hash) -> Option<Transaction> {
        self.transactions.get(&hash).cloned()
    }

    async fn get_transaction_receipt(&self, _hash: Hash) -> Option<TransactionReceipt> {
        // Implementation omitted for brevity
        None
    }

    async fn send_transaction(&self, _transaction: Transaction) -> Hash {
        Hash([0u8; 32])
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
    let balance = client.get_balance(Address([0u8; 32])).await;
    let gas_price = client.gas_price().await;
    println!("Balance: {}, Gas Price: {}", balance, gas_price);
    Ok(())
}

#[tokio::test]
async fn test_get_none_block_by_hash() {
    let blockhead = Blockhead::new(":memory:").unwrap();
    let block_result = blockhead.get_block_by_hash("abcdef".into()).await;
    assert!(block_result.is_ok());
    assert!(block_result.unwrap().is_none());
}

#[tokio::test]
async fn test_get_inserted_block_by_hash() {
    let blockhead = Blockhead::new(":memory:").unwrap();
    let latest_block = blockhead.get_latest_block().await;

    let transaction = Transaction {
        from_address: Address([0; 32]),
        to_address: Address([1; 32]),
        value: 100,
        data: vec![1, 2, 3],
    };
    let block_hash = blockhead.send_transaction(transaction).await;
    let block_result = blockhead.get_block_by_hash(block_hash).await;
    assert!(block_result.is_ok());
    assert!(block_result.unwrap().is_none());
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
