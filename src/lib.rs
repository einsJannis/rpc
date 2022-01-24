use std::fs::read_to_string;
use std::path::Path;

mod lexer;
mod parser;

trait ContentOrigin {
    fn origin_description(&self) -> &str;
    fn content(&self) -> &str;
}

impl ContentOrigin for Path {
    fn origin_description(&self) -> &str {
        self.as_str().unwrap()
    }
    fn content(&self) -> &str {
        read_to_string(self).as_str()
    }
}

#[derive(Clone)]
struct ContentLocation {
    origin: dyn ContentOrigin,
    location: usize
}

trait WithContentLocation {
    fn content_location(&self) -> &ContentLocation;
}
