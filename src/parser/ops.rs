
#[derive(Debug)]
pub enum BinOp {
//  '.'
    Access,
//  "->"
    PtrAccess,
//  '+'
    Add,
//  '-'
    Sub,
//  '*'
    Mul,
//  '/'
    Div,
//  '%'
    Mod,
//  '&'
    And,
//  '|'
    Or,
//  '^'
    Xor,
//  "&&"
    LogAnd,
//  "||"
    LogOr,
//  ">>"
    Shr,
//  "<<"
    Shl,
//  ">"
    Gt,
//  "<"
    Lt,
//  ">="
    Geq,
//  "<="
    Leq,
//  "=="
    Eq,
//  "!="
    Neq
}

#[derive(Debug)]
pub enum UnOp {
//  '*'
    Deref,
//  '&'
    Ref,
//  "++"
    Inc,
//  "--"
    Dec,
//  "~"
    Not,
//  "!"
    LogNot,
}

//  #[rule("?")]
//  #[rule(":")]

// shortcut-assignment = ident, ( "+=",
//                                "-=",
//                                "*=",
//                                "\=",
//                                "%=",
//                                ">>=",
//                                "<<=",
//                                "&=",
//                                "|=",
//                                "^=" ), expression, ";" ;
#[derive(Debug)]
pub enum ShortAssign {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    BOr,
    Xor
}
