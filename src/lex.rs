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

fn check_string(s: &str) -> &str {
    if s.len() < 2 {
        panic!("Invalid string: {s}");
    }

    let b = s.as_bytes();
    let f = b[0] as char;
    let l = b[b.len()-1] as char;

    match f {
        '\'' | '"' if f == l => &s[1..s.len()-1],
        _ => panic!("Invalid string: {s}")
    }
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Lex<T> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.tokenizer.next()?;
        let c = s.chars().next().unwrap();

        match c {
            '-' => Some(if s.len() == 1 {
                    Word("-")
                } else {
                    Num(s.parse().expect("Cannot parse number"))
                }),
            '0'..='9' => Some(Num(s.parse().expect("Cannot parse number"))),
            '[' => Some(BracketLeft),
            ']' => Some(BracketRight),
            '{' => Some(BraceLeft),
            '}' => Some(BraceRight),
            ':' => Some(Assign),
            '\'' | '"' => Some(Str(check_string(s))),
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

    #[test]
    #[should_panic]
    fn invalid_string1() {
        let _result: Vec<_> = lex(r#"'asdf"#).collect();
    }

    #[test]
    #[should_panic]
    fn invalid_string2() {
        let _result: Vec<_> = lex(r#""asdf"#).collect();
    }

    #[test]
    #[should_panic]
    fn invalid_string3() {
        let _result: Vec<_> = lex(r#"'asda""#).collect();
    }
}