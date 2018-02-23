#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate blockchain;
#[macro_use]
extern crate serde_json;

use std::sync::Mutex;
use blockchain::{Blockchain, Transaction, proof_of_work};
use rocket::State;
use rocket_contrib::{Json, Value};

#[get("/chain")]
fn chain(blockchain: State<Mutex<Blockchain>>) -> Json<Value> {
    let blockchain = blockchain.lock().unwrap();
    Json(json!({
        "chain": blockchain.blocks,
        "length": blockchain.len()
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![chain])
        .manage(Mutex::new(Blockchain::new()))
        .launch();
}
