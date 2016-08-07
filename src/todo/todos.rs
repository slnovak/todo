use std::io::prelude::*;
use std::fs::File;
use std::io;

use rustc_serialize::Decodable;
use rustc_serialize::Decoder;
use rustc_serialize::Encodable;
use rustc_serialize::Encoder;
use rustc_serialize::json;

use todo::Todo;

pub enum LoadError {
    FileLoad(io::Error),
    FileRead(io::Error),
    FileDecode(json::DecoderError),
}

impl From<json::DecoderError> for LoadError {
    fn from(err: json::DecoderError) -> LoadError {
        LoadError::FileDecode(err)
    }
}

pub struct Todos<'a> {
    pub todos: Vec<Todo>,
    pub path: &'a str,
}

impl<'a> Todos<'a> {
    fn load(path: &'a str) -> Result<Todos, LoadError> {
        let mut file = try!(File::open(path).map_err(LoadError::FileLoad));

        let mut contents = String::new();

        try!(file.read_to_string(&mut contents).map_err(LoadError::FileRead));

        let decoded_todos: Vec<Todo> = try!(json::decode(&contents));

        Ok(Todos {
            path: path,
            todos: decoded_todos,
        })
    }

    fn save(&self, path: &str) {
        // let mut f = try!(File::create(self.path));
    }
}

impl<'a> Decodable for Todos<'a> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Todos<'a>, D::Error> {
        d.read_seq(|d, item_count| {
            let mut todos: Vec<Todo> = Vec::with_capacity(item_count);

            for i in 0..item_count {
                let todo = try!(d.read_seq_elt(i, |d| Todo::decode(d)));
                todos.push(todo);
            }

            Ok(Todos {
                todos: todos,
                path: "...",
            })
        })
    }
}

impl<'a> Encodable for Todos<'a> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(self.todos.len(), |encoder| {
            for (id, todo) in self.todos.iter().enumerate() {
                try!(encoder.emit_seq_elt(id, |encoder| todo.encode(encoder)));
            }
            Ok(())
        })
    }
}
