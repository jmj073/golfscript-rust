use crate::golf::Golf;

// TODO Eq
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BlockType {
    Str(String),
    BuiltIn(fn (&mut Golf)),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i128),
    Arr(Vec<Value>),
    Str(String), // TODO String -> Vec<char>
    Block(BlockType),
}