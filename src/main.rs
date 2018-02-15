use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate data_encoding;
use data_encoding::HEXLOWER;

extern crate ring;
use ring::digest;

extern crate serde;
use serde::{Serialize, Serializer};
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

fn main() {
    let block = Block {
        index: 1,
        timestamp: timestamp(),
        proof: 1,
        previous_hash: Hash::new(b"hello world"),
        transactions: vec![]
    };

    let block2 = Block {
        index: 2,
        timestamp: timestamp(),
        proof: 1,
        previous_hash: hash_block(&block),
        transactions: vec![]
    };
    println!("{}", serde_json::to_string_pretty(&block).unwrap());
    println!("{}", serde_json::to_string_pretty(&block2).unwrap());
}

struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block {
                index: 1,
                timestamp: timestamp(),
                proof: 1,
                previous_hash: Hash::new(b"first"),
                transactions: vec![]
            }]
        }
    }
}

fn append_block(blockchain: &mut Blockchain) {}

#[derive(Serialize)]
struct Block {
    index: u64,
    timestamp: f64,
    proof: u64,
    previous_hash: Hash,
    transactions: Vec<Transaction>
}

#[derive(Serialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64
}

struct Hash {
    digest: digest::Digest
}

impl Hash {
    fn new(bytes: &[u8]) -> Hash {
        Hash { digest: digest::digest(&digest::SHA256, bytes) }
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = HEXLOWER.encode(self.digest.as_ref());
        serializer.serialize_str(&s)
    }
}

fn hash_block(block: &Block) -> Hash {
    let s = serde_json::to_string(block).unwrap();
    Hash::new(s.as_ref())
}

fn timestamp() -> f64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("uh oh");
    timestamp.as_secs() as f64 + timestamp.subsec_nanos() as f64 * 1e-9
}
