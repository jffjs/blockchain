use std::time::{SystemTime, UNIX_EPOCH};

extern crate ring;
use ring::digest;

extern crate serde;
use serde::{Serialize, Serializer};
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

fn main() {
    println!("{}", timestamp());

}

#[derive(Serialize)]
struct Block {
    index: u64,
    timestamp: f64,
    proof: u64,
    hash: Hash,
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

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bytes(self.digest.as_ref())
    }
}

fn timestamp() -> f64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("uh oh");
    timestamp.as_secs() as f64 + timestamp.subsec_nanos() as f64 * 1e-9
}
