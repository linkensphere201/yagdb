pub trait TokenLiteral {
    fn literal(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_proc_macro::TokenLiteral;

    #[derive(TokenLiteral)]
    enum MyTest {
        Foo,
        Bar,
    }
    #[test]
    fn test_token_literal() {
        let a = MyTest::Foo;
        let b = MyTest::Bar;
        println!("{}", a.literal());
        println!("{}", b.literal());
    }
}
