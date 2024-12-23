
pub enum BinOp {
//  '.'
    Access,
//  "->"
    PtrAccess,
//  '+'
    Add,
//  '_'
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
