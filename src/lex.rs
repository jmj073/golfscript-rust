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

pub type LexResult<T> = Result<T, LexError>;

#[derive(Debug)]
pub enum LexError {
    UnterminatedString,
    InvalidNumber,
}

pub struct Lex<T>
{
    tokenizer: T,
    done: bool
}

impl<T> Lex<T> {
    fn new(tokenizer: T) -> Self {
        Self { tokenizer, done: false }
    }
}

fn lex_string(s: &str) -> LexResult<Token<'_>> {
    if s.len() < 2 {
        panic!("Invalid string: {s}");
    }

    let b = s.as_bytes();
    let f = b[0] as char;
    let l = b[b.len()-1] as char;

    match f {
        '\'' | '"' if f == l => Ok(Str(&s[1..s.len()-1])),
        _ => Err(LexError::UnterminatedString),
    }
}

fn lex_int(s: &str) -> LexResult<Token<'_>> {
    if let Ok(num) = s.parse() {
        Ok(Num(num))
    } else {
        Err(LexError::InvalidNumber)
    }
}

fn lex_token(s: &str) -> LexResult<Token<'_>> {
    let c = s.chars().next().unwrap(); // token이 비어있을리가?!
    Ok(match c {
        '-' => if s.len() == 1 {
                Word("-")
            } else {
                lex_int(s)?
            },
        '0'..='9' => lex_int(s)?,
        '[' => BracketLeft,
        ']' => BracketRight,
        '{' => BraceLeft,
        '}' => BraceRight,
        ':' => Assign,
        '\'' | '"' => lex_string(s)?,
        _ => Word(s),
    })

}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Lex<T> {
    type Item = LexResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let res = lex_token(self.tokenizer.next()?);
        if res.is_err() {
            self.done = true;
        }
        Some(res)
    }
}

pub fn lex<'a>(s: &'a str) -> impl Iterator<Item = LexResult<Token<'a>>> {
    Lex::new(tokenize(s))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_test() {
        let result: Vec<_> =
            lex(r#"+<-3- 45{'a\'b';cd1"#)
            .map(|res| res.unwrap())
            .collect();
        let expect = [
            Word("+"), Word("<"), Num(-3), Word("-"), Word(" "),
            Num(45), BraceLeft, Str(r#"a\'b"#), Word(";"), Word("cd1")
        ];

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn invalid_string1() {
        let _result: Vec<_> = lex(r#"'asdf"#).map(|res| res.unwrap()).collect();
    }

    #[test]
    #[should_panic]
    fn invalid_string2() {
        let _result: Vec<_> = lex(r#""asdf"#).map(|res| res.unwrap()).collect();
    }

    #[test]
    #[should_panic]
    fn invalid_string3() {
        let _result: Vec<_> = lex(r#"'asda""#).map(|res| res.unwrap()).collect();
    }
}