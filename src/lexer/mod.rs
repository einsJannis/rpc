use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::{ContentOrigin, ContentLocation};

#[derive(Clone)]
pub struct Token {
    value: String,
    content_location: ContentLocation
}

impl PartialEq<&str> for Token {
    fn eq(&self, rhs: &&str) -> bool {
        self.value == rhs
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnknownToken(ContentLocation)
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnknownToken(location) => f.write_fmt(format_args!("Found unknown token in {} at index {}", location.0.origin.origin_description(), location.0.location))
        }
    }
}

impl Error for LexerError {}

pub struct Lexer {
    token_definitions: Vec<String>
}

impl Lexer {
    pub fn new(token_definitions: Vec<String>) -> Lexer {
        Lexer { token_definitions }
    }
}

impl Lexer {
    pub fn lex(&self, content_origin: &dyn ContentOrigin) -> Result<Vec<Token>, LexerError> {
        let content = content_origin.content();
        let mut i: usize = 0;
        let mut result: Vec<Token> = Vec::new();
        'outer:while i < content.len() {
            'inner:for token_definition in self.token_definitions {
                if !content.get(i..)
                    .ok_or(|_| panic!("unreachable")).unwrap()
                    .starts_with(token_definition) { continue 'inner }
                i += token_definition.len();
                result += Token { 
                    value: token_definition.clone(),
                    content_location: ContentLocation { location: i, origin: Box::new(content_origin) }
                };
                continue 'outer
            }
            return Err(LexerError::UnknownToken(ContentLocation {
                location: i,
                origin: Box::new(content_orgin)
            }))
        }
        return Ok(result)
    }
}

pub struct TokenIterator {
    tokens: Vec<Token>,
    index: usize,
    index_stack: Vec<usize>
}

impl Iterator for TokenIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tokens.len() { return None }
        let result = self.tokens.index(self.index);
        self.index += 1;
        return Some((*result).clone())
    }
}

impl TokenIterator {
    pub fn new(tokens: Vec<Token>) -> TokenIterator {
        TokenIterator { tokens, index: 0, index_stack: vec![] }
    }
}

impl TokenIterator {
    pub fn push(&mut self) {
        self.index_stack.push(self.index)
    }
    pub fn pop(&mut self) {
        self.index = self.index_stack.pop()?;
    }
    pub fn spop(&mut self) {
        self.index_stack.pop();
    }
    pub fn auto_use<T, F: FnOnce() -> T>(&mut self, function: F) -> T {
        let index = self.index;
        self.index_stack.push(index);
        let result = function();
        if self.index_stack.last()? == index { self.spop() }
        return result
    }
}
