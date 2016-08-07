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
    pub fn from_json(s: &str) -> json::DecodeResult<Todo> {
        json::decode(&s) as json::DecodeResult<Todo>
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
