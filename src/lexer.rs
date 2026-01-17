use std::sync::LazyLock;
use regex::Regex;

const IDENTIFIER_RX_STR: &str = r#"[a-zA-Z_][a-zA-Z0-9_]*"#;
const STRING_RX_STR: &str     = r#"'(?:\\.|[^'])*'?|"(?:\\.|[^"])*"?"#;
const NUM_RX_STR: &str        = r#"-?[0-9]+"#;
const COMMENT_RX_STR: &str    = r#"#[^\n\r]*"#;
static TOKEN_RX_STR: LazyLock<String>  = LazyLock::new(|| [
                                            IDENTIFIER_RX_STR,
                                            STRING_RX_STR,
                                            NUM_RX_STR,
                                            COMMENT_RX_STR,
                                            "."
                                        ].join("|"));

static TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(&TOKEN_RX_STR).unwrap());

pub enum Token {

}

fn tokenize(s: &str) -> impl Iterator<Item = &str> {
    TOKEN_RE.find_iter(s).map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test() {
        let result: Vec<_> = tokenize(r#"+<-3- 45{'a\'b';cd1"#).collect();
        let expect = [
            "+", "<", "-3", "-", " ", "45", "{", r#"'a\'b'"#, ";", "cd1"
        ];

        assert_eq!(result, expect);
    }
}