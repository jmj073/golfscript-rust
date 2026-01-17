use crate::lex::tokenize::tokenize;


mod tokenize;

#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Word(&'a str),
    Str(&'a str),
    Num(i128),
    Block(&'a str),
    BracketLeft,
    BracketRight,
    BraceLeft,
    BraceRight,
    Assign
}

pub struct Lex<T>
{
    tokenizer: T
}

impl<T> Lex<T> {
    fn new(tokenizer: T) -> Self {
        Self { tokenizer }
    }
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Lex<T> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub fn lex<'a>(s: &'a str) -> impl Iterator<Item = Token<'a>> {
    Lex::new(tokenize(s))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_test() {
        let result: Vec<_> = lex(r#"+<-3- 45{'a\'b';cd1"#).collect();
        let expect = [];

        assert_eq!(result, expect);
    }
}