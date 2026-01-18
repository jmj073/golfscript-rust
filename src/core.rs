use core::panic;
use std::sync::LazyLock;

use crate::golf::{Golf, Stack, Env};
use crate::value::{BlockType, Value, Value::*};

// TODO panic -> Result

fn gfn_tilde(g: &mut Golf) {
    let stk = &mut g.stk;

    if let Some(v) = stk.pop() {
        match v {
            Int(n) => stk.push(Int(!n)),
            Arr(mut a) => stk.append(&mut a),
            Str(s) => g.exec(&s),
            Block(b) =>
                match b {
                    BlockType::Str(s) => g.exec(&s),
                    BlockType::BuiltIn(gfn) => gfn(g),
                }
        }
    } else {
        panic!("~: stack is empty!");
    }
}

fn gfn_backtick(g: &mut Golf) {
    unimplemented!();
}

fn gfn_exc(g: &mut Golf) {
    let stk = &mut g.stk;

    if let Some(v) = stk.pop() {
        stk.push(Int(match v {
            Int(n) => (n != 0) as i128,
            Arr(a) => (!a.is_empty()) as i128,
            Str(s) => (!s.is_empty()) as i128,
            Block(b) =>
                match b {
                    BlockType::Str(s) => (!s.is_empty()) as i128,
                    BlockType::BuiltIn(_) => 1,
                }
        }));
    } else {
        panic!("!: stack is empty!");
    }
}

fn gfn_at(g: &mut Golf) {
    let stk = &mut g.stk;

    if stk.len() < 3 {
        panic!("@: stack size < 3!");
    }

    let len = stk.len();
    stk.swap(len-1, len-3);
}

fn gfn_dollar(g: &mut Golf) {
    let stk = &mut g.stk;

    if let Some(v) = stk.pop() {
        match v {
            Int(n) => {
                let n = n as usize;
                if n < stk.len() {
                    stk.push(stk[stk.len()-n].clone());
                }
            },
            Str(s) => {
                let mut vec: Vec<char> = s.chars().collect();
                vec.sort();
                stk.push(Str(String::from_iter(vec.iter())));
            },
            Arr(a) => { // TODO 문자열 버전도
                let mut a: Vec<_> = a
                    .into_iter()
                    .map(|v|
                        if let Int(n) = v {
                            n
                        } else {
                            panic!("$: cannot sort!")
                        })
                    .collect();
                a.sort();
                stk.push(Arr(a.into_iter().map(Int).collect()));
            },
            Block(b) => unimplemented!()
        }
    } else {
        panic!("$: stack is empty!");
    }
}

fn gfn_plus(g: &mut Golf) {
    let stk = &mut g.stk;

    let b = stk.pop().unwrap();
    let a = stk.pop().unwrap();

    match (a, b) {
        (Int(n1), Int(n2)) => {
            stk.push(Int(n1 + n2));
        },
        _ => unimplemented!()
    }
}

fn gfn_space(_g: &mut Golf) {

}

pub static CORE_BLOCKS: LazyLock<Env> = LazyLock::new(
|| Env::from_iter([
    (String::from("~"), Block(BlockType::BuiltIn(gfn_tilde))),
    (String::from("`"), Block(BlockType::BuiltIn(gfn_backtick))),
    (String::from("!"), Block(BlockType::BuiltIn(gfn_exc))),
    (String::from("@"), Block(BlockType::BuiltIn(gfn_at))),
    (String::from("$"), Block(BlockType::BuiltIn(gfn_dollar))),
    (String::from("+"), Block(BlockType::BuiltIn(gfn_plus))),
    (String::from(" "), Block(BlockType::BuiltIn(gfn_space))),
].into_iter()));

#[cfg(test)]
mod tests {

}