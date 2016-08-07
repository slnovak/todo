extern crate todo;

use todo::{Todo};

fn main() {
    let todo = Todo::from_json("{\"description\":\"Hello world\"}");

    match todo {
        Ok(todo) => println!("Description: {}", todo.description),
        Err(err) => println!("Unable to parse!"),
    }
}
