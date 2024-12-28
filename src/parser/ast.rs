use ilex::rule::Sign;

use super::ops::*;

// (* NOTE: Tokens can be found in the spec file *)
// translation-unit = declaration*
pub struct File(pub Vec<Declaration>);

// declaration = function-declaration
//             | function-definition
//             | type-definiton
//             | static-variable-declaration
//             ;
pub enum Declaration {
    TypeDefinition(TypeDefinition),
    FunctionDeclaration(FunctionDeclaration),
    FunctionDefinition(FunctionDefinition),
    StaticVariableDeclaration(StaticVariableDeclaration)
}

// type-specifier = "u8"
//                | "u16"
//                | "u32"
//                | "u64"
//                | "usize"
//                | "i8"
//                | "i16"
//                | "i32"
//                | "i64"
//                | "isize"
//                | "z8"
//                | "z16"
//                | "z32"
//                | "z64"
//                | "zsize"
//                | "f32"
//                | "f64"
//                | "c32"
//                | "c64"
//                | "bool"
//                | "struct", ident,
//                | "enum", ident,
//                | "union", ident,
//                | tuple-def
//                | user-type (* pretty sure this is context dependent *)
//                | type-specifier, "*", (* NOTE: ptrs :) *) ;
#[derive(Clone)]
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
    // tuple-def = "(", [ type-specifier ], { ",", type-specifier }, ")", ";" ;
    Tuple(Vec<Type>),
    // array-def = type-specifier, [ "[", number, "]", ], { "[", number, "]", };
    Array(Box<Type>, usize),
    Struct(String),
    Enum(String),
    Union(String),
    Pointer(Box<Type>),
}

// type-definition = (( "struct", ident, "{", { ident, type-specifier, ";" }, "}" )
//                  | ( "union",  ident, "{", { ident, type-specifier, ";" }, "}" )
//                  | ( "enum",   ident, "{", { ident, type-specifier, ";" }, "}" )
//                  | "typedef", type-specifier, ident ), ";" ;
pub enum TypeDefinition {
    StructDef {
        ident: String,
        fields: Vec<(String, Type)>,
    },
    UnionDef {
        ident: String,
        fields: Vec<(String, Type)>,
    },
    EnumDef {
        ident: String,
        values: Vec<(String, Option<Type>)>,
    },
    TypeDef {
        old_type: Box<Type>, 
        alias: String,
    },
}

// type-instantiation = ( expression
//                      | array-instantiation
//                      | tuple-instantiation
//                      | struct-instantiation
//                      | enum-instantiation
//                      | { "&" }, ident
//                      ) ;

pub enum TypeInstantiation {
    Expr(Expression),
    Array(ArrayInstantiation),
    Tuple(TupleInstantiation),
    Struct(StructInstantiation),
    Enum(EnumInstantiation),
    Pointer(PointerInstantiation)
}

// expression = ( binary-op
//       | unary-op
//       | ident
//       | function-call
//       | conditional   (* conditionals are binops or unops, so ignore *)
//       | literal       (* number literal *)
//       | "(", expression, ")"),;

pub enum Expression {
    // binary-op = expression, binop, expression ;
    BinOp {
        operation: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // unary-op  = unop, ( ident | literal )
    //           | unop, "(", expression, ")" ;
    UnOp {
        operation: UnOp,
        operand: Box<Expression>
    },
    Ident(String),
    // function-call = ident, "(", [ expression ], { ",", expression }, ")" ;
    FunctionCall {
        name: String,
        arguments: Vec<Expression>
    },
    Literal(Literal)
}
 
// literal = string-literal
//         | ptr-literal
//         | int-literal
//         | float-literal
//         | gaussian-literal (* I'll do these later :( *)
//         | complex-literal
//         | bool-literal ;
pub enum Literal {
    String(String),
    Integer {
        sign: Sign,
        num: usize,
    },
    Float {
        sign: Sign,
        num: f64
    },
    Bool(bool),
    Ptr,
} 

// array-instantiation = "[", [ type-instantiation ], { ",", type-instantiation }, "]" ;
pub struct ArrayInstantiation {
    values: Vec<TypeInstantiation>,
}

// tuple-instantiation = "(", [ type-instantiation ], { ",", type-instantiation }, ")" ;
pub struct TupleInstantiation {
    values: Vec<TypeInstantiation>,
}
 
// struct-instantiation = "{", { ident, "=", type-instantiation }, "}" ;
pub struct StructInstantiation {
    fields: Vec<(String, TypeInstantiation)>,
}

// enum-instantiation   = ident, [ type-instantiation ] ;
pub struct EnumInstantiation {
    discriminator: String,
    value: Option<Box<TypeInstantiation>>
}

// pointer-instantiation = "null" | ( "&", ident ) | int-literal ;
pub enum PointerInstantiation {
    Null,
    Reference {
        ident: String,
    },
    Value {
        value: usize
    }
}

// variable-declaration = 
//     type-specifier, ident [ "=", type-instantiation ], { ",", ident, [ "=", type-instantiation ] }, ";" ;

pub struct VariableDeclaration {
    pub var_type: Type, 
    pub ident: String,
    pub definition: Option<TypeInstantiation>,
    pub extra: Vec<(String, Option<TypeInstantiation>)>
}

// static-variable-declaration = "static", variable-declaration ;

pub struct StaticVariableDeclaration {
    pub declaration: VariableDeclaration
}

// function-signature =
//     type-specifier, ident, "(", [ type-specifier, ident ], { ",", type-specifier, ident }, ")" ;
//
// function-declaration = 
//     function-signature, ";" ;
pub struct FunctionDeclaration {
    return_type: Type,
    ident: String,
    arguments: Vec<(Type, String)>
}

// code-block-body = statement* ;

pub struct CodeBlock {
    code: Vec<Statement>
}

// function-definition = function-signature, code-block-body ;

pub struct FunctionDefinition {
    declaration: FunctionDeclaration,
    code: CodeBlock
}

// variable-assignment = ident, "=", expression, ";" ;
pub struct VariableAssignment {
    ident: String,
    expr: Expression,
}

// statement = variable-declaration
//           | variable-assignment
//           | shortcut-assignment
//           | if-statement
//           | while-statement
//           | for-statement
//           | return-statement
//           | match-statement
//           | if-let-statement ;
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    VariableAssignment(VariableAssignment),
    // shortcut-assignment = ident, ( short-assign ), expression, ";" ;
    ShortcutAssignment {
        ident: String,
        shortcut: ShortAssign,
        expr: Expression,
    },
    // if-statement = "if", "(", expression, ")", "{", code-block-body, "}",
    //      { "else", "if", "(", expression, ")", "{", code-block-body, "}" },
    //      [ "else", "{", code-block-body, "}" ] ;
    IfStatement {
        condition: Expression,
        code_block: CodeBlock,
        else_if: Option<Vec<(Expression, CodeBlock)>>,
        else_ex: Option<CodeBlock>
    },
    // while-statement = "while", "(", expression, ")", "{", code-block-body, "}" ;
    WhileStatement {
        condition: Expression,
        code_block: CodeBlock,
    },
    // for-statement   = "for", "(", [ variable-assignment ], ";", 
    //                               [ expression ],  ";", 
    //                               [ expression ], ")", "{", code-block-body, "}" ;
    ForStatement {
        initialization: Option<VariableAssignment>,
        condition: Expression,
        continuation: Expression
    },
    // return-statement = "return", expression, ";" ;
    ReturnStatement {
        expr: Expression,
    },
    // match-statement  = "match", expression, "{", { match-clause }, "}" ;
    MatchStatement {
        pattern: Expression,
        clauses: Vec<MatchClause>
    },
    // if-let-statement = "if", "let", type-instantiation, "=" expression, "{", code-block-body, "}"
    //                  [ "else", "{", code-block-body, "}", ] ;
    IfLetStatement {
        pattern: TypeInstantiation,
        matching: Expression,
        code_block: CodeBlock,
        else_block: Option<CodeBlock>
    }
}

// match-clause     = type-instantiation, ":", "{", code-block-body, "}", "," ;
pub struct MatchClause {
    pattern: TypeInstantiation,
    code: CodeBlock
}
