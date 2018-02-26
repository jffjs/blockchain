extern crate byteorder;
extern crate data_encoding;
extern crate ring;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::cmp::PartialEq;
use std::time::{SystemTime, UNIX_EPOCH};
use byteorder::{ByteOrder, LittleEndian};
use data_encoding::HEXLOWER;
use ring::digest;
use serde::{Serialize, Serializer};

pub struct Node {
    pub blockchain: Vec<Block>,
    pub transactions: Vec<Transaction>
}

impl Node {
    pub fn new() -> Node {
        let blockchain = vec![
            Block::new(1, 100, Hash::new(b"first"), vec![])
        ];
        let transactions = vec![];

        Node { blockchain, transactions }
    }

    pub fn len(&self) -> usize {
        self.blockchain.len()
    }

    pub fn add_transaction(&mut self, trans: Transaction) -> u64{
        self.transactions.push(trans);
        self.last_block().index + 1
    }

    pub fn new_block(&mut self, proof: u64, last_hash: Hash) -> &Block {
        let transactions = self.transactions.drain(..).collect();
        let block = Block::new(self.len() as u64 + 1,
                               proof,
                               last_hash,
                               transactions);
        self.blockchain.push(block);
        self.last_block()
    }

    pub fn last_block(&self) -> &Block {
        &self.blockchain[self.blockchain.len() - 1]
    }

    pub fn valid_chain(&self) -> bool {
        let mut index = 1;
        while index < self.len() {
            let last_block = &self.blockchain[index -1];
            let block = &self.blockchain[index];
            if block.previous_hash != last_block.hash() {
                return false;
            }

            if !valid_proof(last_block.proof, block.proof) {
                return false;
            }
            index += 1;
        }

        true
    }
}

#[derive(Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: f64,
    pub proof: u64,
    pub previous_hash: Hash,
    pub transactions: Vec<Transaction>
}

impl Block {
    fn new(index: u64,
           proof: u64,
           previous_hash: Hash,
           transactions: Vec<Transaction>) -> Block {
        Block { index, proof, previous_hash, transactions, timestamp: timestamp() }
    }

    pub fn hash(&self) -> Hash {
        let s = serde_json::to_string(self).unwrap();
        Hash::new(s.as_ref())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f64
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: f64) -> Transaction {
        Transaction { sender: sender.to_owned(),
                      recipient: recipient.to_owned(),
                      amount }
    }
}

pub struct Hash {
    pub digest: digest::Digest
}

impl Hash {
    fn new(bytes: &[u8]) -> Hash {
        Hash { digest: digest::digest(&digest::SHA256, bytes) }
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Hash) -> bool {
        self.digest.as_ref() == other.digest.as_ref()
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

fn timestamp() -> f64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("uh oh");
    timestamp.as_secs() as f64 + timestamp.subsec_nanos() as f64 * 1e-9
}

pub fn proof_of_work(last_proof: u64) -> u64 {
    let mut proof = 0;
    while !valid_proof(last_proof, proof) {
        proof += 1;
    }
    proof
}

pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let difficulty = 2;

    let mut guess_buf = [0; 16];
    LittleEndian::write_u64(&mut guess_buf, last_proof);
    LittleEndian::write_u64(&mut guess_buf[8..], proof);

    let guess_hash = digest::digest(&digest::SHA256, &guess_buf);
    let bytes = guess_hash.as_ref();

    for i in 0..difficulty {
        if bytes[i] != 0 {
            return false;
        }
    }
    true
}
