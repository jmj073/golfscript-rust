use crate::value::{Value, BlockType};
use crate::lex::{lex, Token};

pub type Env = std::collections::HashMap<String, Value>;
pub type Stack = Vec<Value>;

pub struct Golf {
    pub env: Env,
    pub stk: Stack,
}

impl Golf {
    pub fn new() -> Self {
        Self {
            env: Env::new(),
            stk: Stack::new(),
        }
    }

    pub fn exec(&mut self, s: &str) {
        let l = lex(s);

        for res in l {
            match res {
                Ok(tok) =>
                    match tok {
                        Token::Word(s) => self.process_word(s),
                        Token::Str(s) =>
                            self.stk.push(Value::Str(String::from(s))),
                        Token::Num(n) => self.stk.push(Value::Int(n)),
                        _ => unimplemented!(),
                    },
                Err(e) => { // TODO 출력이 아니라 반환으로 바꾸기!
                    eprintln!("{:?}", e);
                    return;
                }
            }

        }
    }

    fn process_word(&mut self, s: &str) {
        let v = self.env.get(s);

        if let Some(v) = v {
            let stk = &mut self.stk;
            match v {
                Value::Block(b) =>
                    match b {
                        BlockType::BuiltIn(gfn) => gfn(self),
                        BlockType::Str(s) => self.exec(&s.clone()),
                    }
                _ => stk.push(v.clone()),
            }
        } else {
            panic!("Cannot find variable: {s}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::CORE_BLOCKS;

    #[test]
    fn golf_plus() {
        let mut g = Golf::new();
        g.env = CORE_BLOCKS.clone();

        g.exec("3 4+");

        assert_eq!(g.stk[0], Value::Int(7));
    }
}
