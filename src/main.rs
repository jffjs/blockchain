use std::time::{SystemTime, UNIX_EPOCH};

extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};

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
    let mut blockchain = Blockchain::new();
    blockchain.transactions.push(Transaction::new("foo", "bar", 5.1));
    blockchain.new_block(200);
    blockchain.new_block(300);

    let proof = proof_of_work(100);
    println!("{}", proof);

    println!("{}", serde_json::to_string_pretty(&blockchain.blocks).unwrap());
}

struct Blockchain {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            blocks: vec![
                Block::new(1, 100, Hash::new(b"first"), vec![])
            ],
            transactions: vec![]
        }
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }

    fn new_block(&mut self, proof: u64) -> &Block {
        let transactions = self.transactions.drain(..).collect();
        let block = Block::new(self.len() as u64 + 1,
                               proof,
                               self.blocks[self.len() - 1].hash(),
                               transactions);
        self.blocks.push(block);
        &self.blocks[self.len() -1]
    }
}

#[derive(Serialize)]
struct Block {
    index: u64,
    timestamp: f64,
    proof: u64,
    previous_hash: Hash,
    transactions: Vec<Transaction>
}

impl Block {
    fn new(index: u64,
           proof: u64,
           previous_hash: Hash,
           transactions: Vec<Transaction>) -> Block {
        Block { index, proof, previous_hash, transactions, timestamp: timestamp() }
    }

    fn hash(&self) -> Hash {
        let s = serde_json::to_string(self).unwrap();
        Hash::new(s.as_ref())
    }
}

#[derive(Serialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f64
}

impl Transaction {
    fn new(sender: &str, recipient: &str, amount: f64) -> Transaction {
        Transaction { sender: sender.to_owned(),
                      recipient: recipient.to_owned(),
                      amount }
    }
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

fn timestamp() -> f64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("uh oh");
    timestamp.as_secs() as f64 + timestamp.subsec_nanos() as f64 * 1e-9
}

fn proof_of_work(last_proof: u64) -> u64 {
    let mut proof = 0;
    while !valid_proof(last_proof, proof) {
        proof += 1;
    }
    proof
}

fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let mut guess_buf = [0; 16];
    LittleEndian::write_u64(&mut guess_buf, last_proof);
    LittleEndian::write_u64(&mut guess_buf[8..], proof);
    let guess_hash = digest::digest(&digest::SHA256, &guess_buf);
    guess_hash.as_ref()[..3] == [0;4]
}
