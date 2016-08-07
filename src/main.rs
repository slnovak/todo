extern crate todo;

extern crate rustc_serialize;
use rustc_serialize::json;

use todo::{Todo};

fn main() {
    let todo = Todo{description: "Hello world".to_string()};

    match json::encode(&todo) {
        Ok(s) => println!("Encoded: {}", &s),
        Err(err) => println!("Error: {}", err),
    }
}
