extern crate rustc_serialize;

use rustc_serialize::Decodable;
use rustc_serialize::Decoder;
use rustc_serialize::Encodable;
use rustc_serialize::Encoder;
use rustc_serialize::json;

pub struct Todo {
    pub description: String,
}

impl Todo {
    pub fn from_json(content: &str) -> json::DecodeResult<Todo> {
        json::decode(&content)
    }

    pub fn to_json(&self) -> json::EncodeResult<String> {
        json::encode(&self)
    }
}

impl Decodable for Todo {
    fn decode<D: Decoder>(d: &mut D) -> Result<Todo, D::Error> {
        d.read_struct("Todo", 1, |d| {
            let description = try!(d.read_struct_field("description", 0, |d| d.read_str()));
            Ok(Todo { description: description })
        })
    }
}

impl Encodable for Todo {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Todo", 1, |s| {
            try!(s.emit_struct_field("description", 0, |s| s.emit_str(&self.description)));
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use super::Todo;

    #[test]
    fn test_from_json() {
        let todo = Todo::from_json("{\"description\":\"Hello world\"}").unwrap();
        assert_eq!(todo.description, "Hello world".to_string());
    }

    fn test_to_json() {
        let todo = Todo { description: "Hello world".to_string() };
        let json = todo.to_json().unwrap();
        assert_eq!(json, "{\"description\":\"Hello world\"}".to_string());
    }
}
