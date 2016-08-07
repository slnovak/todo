use std::io::prelude::*;
use std::fs::File;
use std::io;

use rustc_serialize::json;

use todo::Todo;

pub struct Todos<'a> {
    pub todos: Vec<Todo>,
    pub path: &'a str,
}

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
