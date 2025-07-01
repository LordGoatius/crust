#![expect(unused_imports)]
#![expect(dead_code)]
use std::collections::HashSet;
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
        v: Variable,
        op: BinOp,
        a1: Variable,
        a2: Variable,
    },
    Triple {
        v: Variable,
        op: UnOp,
        a: Variable,
    },
    Br(Condition, Label),
    Phi(Variable, Vec<Variable>),
}

pub struct CodeBlock {
    instrs: Vec<Instr>,
}
