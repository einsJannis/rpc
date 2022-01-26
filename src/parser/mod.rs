use crate::lexer::TokenIterator;
use crate::{ContentLocation, WithContentLocation};

#[derive(Debug)]
pub enum ParseError {
    NoTokensLeft,
    UnexpectedToken { location: ContentLocation, expected: String },
    FailedToMatchPattern { location: ContentLocation, pattern_name: String }
}

pub trait Pattern {
    type Output;
    fn match_pattern(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self::Output), ParseError>;
}

pub trait Parsable {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError>;
}

impl<T> Pattern for T where T: Parsable {
    type Output = Self;
    fn match_pattern(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self::Output), ParseError> {
        <Self as Parsable>::parse(tokens)
    }
}
