use crate::lexer::TokenIterator;
use crate::WithContentLocation;

trait Pattern {
    type Output: WithContentLocation;
    fn match_pattern(&self, tokens: TokenIterator) -> Self::Output;
}
