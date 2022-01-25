use crate::lexer::TokenIterator;
use crate::{ContentLocation, WithContentLocation};

#[derive(Debug)]
pub enum ParseError {
    NoTokensLeft,
    UnexpectedToken { location: ContentLocation, expected: String },
    FailedToMatchPattern { location: ContentLocation, pattern_name: str }
}

pub trait Pattern {
    type Output: WithContentLocation;
    fn match_pattern(tokens: TokenIterator) -> Result<Self::Output, ParseError>;
}

pub trait Parsable: WithContentLocation {
    fn parse(tokens: TokenIterator) -> Result<Self, ParseError>;
}

impl<T> Pattern for T where T: Parsable {
    type Output = Self;
    fn match_pattern(tokens: TokenIterator) -> Result<Self::Output, ParseError> {
        <Self as Parsable>::parse(tokens)
    }
}
