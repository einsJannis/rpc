use crate::lexer::TokenIterator;
use crate::WithContentLocation;

pub trait Pattern {
    type Output: WithContentLocation;
    fn match_pattern(&self, tokens: TokenIterator) -> Self::Output;
}
