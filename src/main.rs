extern crate todo;

extern crate rustc_serialize;
use rustc_serialize::json;

use todo::{Todo};

fn main() {
    let todo = Todo::from_json("{\"description\":\"Hello world\"}");
}
