use crate::lexer::TokenIterator;
use crate::WithContentLocation;

pub trait Pattern {
    type Output: WithContentLocation;
    fn match_pattern(tokens: TokenIterator) -> Self::Output;
}

pub trait Parsable: WithContentLocation {
    fn parse(tokens: TokenIterator) -> Self;
}

impl<T> Pattern for T where T: Parsable {
    type Output = Self;
    fn match_pattern(tokens: TokenIterator) -> Self::Output { <Self as Parsable>::parse(tokens) }
}
