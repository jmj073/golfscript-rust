use tokenize::tokenize;
use Token::*;


mod tokenize;

#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Word(&'a str),
    Str(&'a str),
    Num(i128),
    BracketLeft,
    BracketRight,
    BraceLeft,
    BraceRight,
    Assign,
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
        let s = self.tokenizer.next()?;
        let c = s.chars().next()?;

        match c {
            '-' => Some(if s.len() == 1 { Word("-") } else { Num(s.parse().ok()?) }),
            '0'..='9' => Some(Num(s.parse().ok()?)),
            '[' => Some(BracketLeft),
            ']' => Some(BracketRight),
            '{' => Some(BraceLeft),
            '}' => Some(BraceRight),
            ':' => Some(Assign),
            '\'' | '"' => Some(Str(&s[1..s.len()-1])),
            _ => Some(Word(s)),
        }
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
        let expect = [
            Word("+"), Word("<"), Num(-3), Word("-"), Word(" "),
            Num(45), BraceLeft, Str(r#"a\'b"#), Word(";"), Word("cd1")
        ];

        assert_eq!(result, expect);
    }
}