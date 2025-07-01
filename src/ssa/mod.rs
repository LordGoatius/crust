#![expect(unused_imports)]
#![expect(dead_code)]
use std::collections::HashSet;
use std::hash::Hash;
use crate::parser::{ast::{}, ops::{BinOp, UnOp}};

pub mod cfg;

pub struct Label(usize);

pub enum Type {
    U8,
    U16,
    U32,
    U64,
    USize,
    I8,
    I16,
    I32,
    I64,
    ISize,
    Z8,
    Z16,
    Z32,
    Z64,
    ZSize,
    F32,
    F64,
    C32,
    C64,
    Bool,
    Void,
    Tuple(Vec<Type>),
    Array(Box<Type>, Vec<usize>),
    Struct(Vec<(String, Type)>),
    Enum(Vec<(String, Option<Type>)>),
    Union(Vec<(String, Type)>),
    Pointer(Box<Type>, usize),
}

pub struct Variable(String);

pub struct StaticValue(Variable, u64, Type);

// A function is a named section of code with specifc call/return requirements.
// These requirements are handled during ir lowering to machine code.
pub struct Function {
    name: String,
    label: Label,
    ret: Type,
    args: Vec<StaticValue>,
    blocks: HashSet<CodeBlock>,
}

pub enum Condition {
    Ne,
    Eq,
    Gt,
    Lt,
    Geq,
    Leq,
}

pub enum Instr {
    Quad {
        v: StaticValue,
        op: BinOp,
        a1: StaticValue,
        a2: StaticValue,
    },
    Triple {
        v: StaticValue,
        op: UnOp,
        a: StaticValue,
    },
    Call(StaticValue, Function),
    Return(Option<StaticValue>),
    Alloca(StaticValue, Type),
    Br(Condition, Label),
    Phi(StaticValue, Vec<StaticValue>),
}

pub struct CodeBlock {
    label: Label,
    instrs: Vec<Instr>,
}
