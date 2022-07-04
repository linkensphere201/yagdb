use super::*;
use common_proc_macro::TokenLiteral;
use logos::Logos;
use std::ops::Range;

#[derive(Debug, Clone, Getters)]
pub struct Token<'s> {
    ty: TokenType,
    text: &'s str,
    span: Range<usize>,
}

#[derive(Debug, Getters)]
pub struct Tokenizer<'a> {
    stmt: &'a str,
    tokens: Vec<Token<'a>>,
}

type Result<'a, T> = std::result::Result<T, LexError<'a>>;

impl<'a> Tokenizer<'a> {
    pub fn try_new(stmt: &'a str) -> Result<'a, Self> {
        let tokens = Self::tokenize(stmt)?;
        Ok(Self { stmt, tokens })
    }
}

impl<'a> Tokenizer<'a> {
    fn tokenize(stmt: &'a str) -> Result<Vec<Token<'a>>> {
        let mut tokens = Vec::with_capacity(64);
        let mut lex = TokenType::lexer(stmt);
        while let Some(t) = lex.next() {
            match t {
                TokenType::Error => {
                    return Err(LexError {
                        source: stmt,
                        span: lex.span(),
                    });
                }
                _ => tokens.push(Token {
                    ty: t,
                    text: lex.slice(),
                    span: lex.span(),
                }),
            }
        }
        Ok(tokens)
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, Hash, TokenLiteral)]
pub enum TokenType {
    #[error]
    Error,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r#"[a-zA-Z][a-zA-Z0-9]*"#)]
    Ident,

    #[regex(r#"`[^`]*`"#)]
    #[regex(r#""([^"\\]|\\.|"")*""#)]
    #[regex(r#"'([^'\\]|\\.|'')*'"#)]
    QuotedString,

    #[regex(r"[0-9]+")]
    LiteralInteger,

    #[regex(r"[0-9]+e[+-]?[0-9]+")]
    #[regex(r"([0-9]*\.[0-9]+(e[+-]?[0-9]+)?)|([0-9]+\.[0-9]*(e[+-]?[0-9]+)?)")]
    LiteralFloat,

    #[token("==")]
    DoubleEq,
    #[token("=")]
    Eq,
    #[token("<>")]
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Lte,
    #[token(">=")]
    Gte,
    #[token("+")]
    Plus,
    #[token("-")]
    DashMinus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("->")]
    RArrow,
    #[token("<-")]
    LArrow,

    // keywords
    #[token("ALL", ignore(ascii_case))]
    ALL,
    #[token("ASC", ignore(ascii_case))]
    ASC,
    #[token("ASCENDING", ignore(ascii_case))]
    ASCENDING,
    #[token("BY", ignore(ascii_case))]
    BY,
    #[token("CREATE", ignore(ascii_case))]
    CREATE,
    #[token("DELETE", ignore(ascii_case))]
    DELETE,
    #[token("DESC", ignore(ascii_case))]
    DESC,
    #[token("DESCENDING", ignore(ascii_case))]
    DESCENDING,
    #[token("DETACH", ignore(ascii_case))]
    DETACH,
    #[token("EXISTS", ignore(ascii_case))]
    EXISTS,
    #[token("LIMIT", ignore(ascii_case))]
    LIMIT,
    #[token("MATCH", ignore(ascii_case))]
    MATCH,
    #[token("MERGE", ignore(ascii_case))]
    MERGE,
    #[token("ON", ignore(ascii_case))]
    ON,
    #[token("OPTIONAL", ignore(ascii_case))]
    OPTIONAL,
    #[token("ORDER", ignore(ascii_case))]
    ORDER,
    #[token("REMOVE", ignore(ascii_case))]
    REMOVE,
    #[token("RETURN", ignore(ascii_case))]
    RETURN,
    #[token("SET", ignore(ascii_case))]
    SET,
    #[token("SKIP", ignore(ascii_case))]
    SKIP,
    #[token("WHERE", ignore(ascii_case))]
    WHERE,
    #[token("WITH", ignore(ascii_case))]
    WITH,
    #[token("UNION", ignore(ascii_case))]
    UNION,
    #[token("UNWIND", ignore(ascii_case))]
    UNWIND,
    #[token("AND", ignore(ascii_case))]
    AND,
    #[token("AS", ignore(ascii_case))]
    AS,
    #[token("CONTAINS", ignore(ascii_case))]
    CONTAINS,
    #[token("DISTINCT", ignore(ascii_case))]
    DISTINCT,
    #[token("ENDS", ignore(ascii_case))]
    ENDS,
    #[token("IN", ignore(ascii_case))]
    IN,
    #[token("IS", ignore(ascii_case))]
    IS,
    #[token("NOT", ignore(ascii_case))]
    NOT,
    #[token("OR", ignore(ascii_case))]
    OR,
    #[token("STARTS", ignore(ascii_case))]
    STARTS,
    #[token("XOR", ignore(ascii_case))]
    XOR,
    #[token("FALSE", ignore(ascii_case))]
    FALSE,
    #[token("TRUE", ignore(ascii_case))]
    TRUE,
    #[token("NULL", ignore(ascii_case))]
    NULL,
    #[token("CONSTRAINT", ignore(ascii_case))]
    CONSTRAINT,
    #[token("DO", ignore(ascii_case))]
    DO,
    #[token("FOR", ignore(ascii_case))]
    FOR,
    #[token("REQUIRE", ignore(ascii_case))]
    REQUIRE,
    #[token("UNIQUE", ignore(ascii_case))]
    UNIQUE,
    #[token("CASE", ignore(ascii_case))]
    CASE,
    #[token("WHEN", ignore(ascii_case))]
    WHEN,
    #[token("THEN", ignore(ascii_case))]
    THEN,
    #[token("ELSE", ignore(ascii_case))]
    ELSE,
    #[token("END", ignore(ascii_case))]
    END,
    #[token("MANDATORY", ignore(ascii_case))]
    MANDATORY,
    #[token("SCALAR", ignore(ascii_case))]
    SCALAR,
    #[token("OF", ignore(ascii_case))]
    OF,
    #[token("ADD", ignore(ascii_case))]
    ADD,
    #[token("DROP", ignore(ascii_case))]
    DROP,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let cyphers = vec![
            "match (a: Person) return a;",
            "create (a:Person{ id: \"1\", age: 15, name: 'Mike', subject: 'Nxp' });",
        ];

        for stmt in cyphers {
            let t = Tokenizer::try_new(stmt).unwrap();
            println!("\n==== stmt: {} ====\n", t.stmt());
            t.tokens().iter().for_each(|t| {
                println!("{:?}", t);
                println!("ty literal: {}", t.ty().literal());
                println!("--------");
            });
        }
    }
}
