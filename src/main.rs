#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate blockchain;
#[macro_use]
extern crate serde_json;
extern crate uuid;

use std::sync::Mutex;
use blockchain::{Node, Transaction, proof_of_work};
use rocket::State;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use uuid::Uuid;

#[get("/chain")]
fn chain(node: State<Mutex<Node>>) -> Json<Value> {
    let node = node.lock().unwrap();
    Json(json!({
        "chain": node.blockchain,
        "length": node.len()
    }))
}

#[post("/transaction", data = "<transaction>")]
fn new_transaction(transaction: Json<Transaction>,
                   node: State<Mutex<Node>>) -> status::Created<Json<Value>> {
    let transaction = transaction.0;
    let mut node = node.lock().unwrap();
    let index = node.add_transaction(transaction);
    status::Created("".to_string(), Some(Json(json!({
        "message": format!("Transaction will be added to Block {}", index)
    }))))
}

#[post("/mine")]
fn mine(b: State<Mutex<Node>>, node_id: State<Uuid>) -> Json<Value> {
    let last_proof;
    let last_hash;
    {
        let node = b.lock().unwrap();
        let last_block = node.last_block();
        last_proof = last_block.proof;
        last_hash = last_block.hash();
    }

    let proof = proof_of_work(last_proof);

    let mut node = b.lock().unwrap();
    let address = node_id.hyphenated().to_string();
    node.add_transaction(Transaction::new("0", &address, 1.0));
    let block = node.new_block(proof, last_hash);

    Json(json!({
        "message": "New block forged",
        "block": block
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![chain, new_transaction, mine])
        .manage(Uuid::new_v4())
        .manage(Mutex::new(Node::new()))
        .launch();
}
