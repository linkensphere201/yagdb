use std::ops::Range;

use peg::{Parse, ParseLiteral, RuleResult};

use super::*;

pub struct FlatTokenStream<'a> {
    tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct Pos(Range<usize>, usize);

impl std::fmt::Display for Pos {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?} ({})", self.0, self.1)
    }
}

impl<'a> Parse for FlatTokenStream<'a> {
    type PositionRepr = Pos;
    fn start(&self) -> usize {
        0
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.tokens.len()
    }

    fn position_repr(&self, pos: usize) -> Pos {
        Pos(self.tokens[pos].span().clone(), pos)
    }
}

impl<'a> ParseLiteral for FlatTokenStream<'a> {
    fn parse_string_literal(&self, pos: usize, literal: &str) -> RuleResult<()> {
        match self.tokens.get(pos) {
            Some(i) => match i.ty() {
                TokenType::QuotedString if literal.eq("QuotedString") => {
                    return RuleResult::Matched(pos + 1, ());
                }
                _ => {}
            },
            None => {}
        }
        RuleResult::Failed
    }
}

#[derive(Debug)]
pub struct SampleQuotedStrAst {
    st_pos: usize,
    ed_pos: usize,
}

peg::parser! {
    grammar test_grammer<'a>() for FlatTokenStream<'a> {
        pub rule quote_string() -> SampleQuotedStrAst
            = s:position!() "QuotedString" e:position!() { SampleQuotedStrAst { st_pos: s, ed_pos: e }  }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peg() {
        let t = Tokenizer::try_new("\"sample\"").unwrap();
        let a = FlatTokenStream {
            tokens: t.tokens().clone(),
        };

        let r = test_grammer::quote_string(&a).unwrap();
        println!("r: {:?}", r);
    }
}
