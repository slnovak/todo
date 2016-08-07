extern crate todo;

use todo::{Todo};

fn main() {
    let todo = try!(Todo::from_json("{\"description\":\"Hello world\"}"));
    println!("Description: {}", todo.description);
}
