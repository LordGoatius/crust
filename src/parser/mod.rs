pub mod ast;
pub mod ops;

use ast::*;
use ilex::{
    token::{self, Cursor, Stream},
    Context, Lexeme, Report,
};

use crate::lexer::Crust;

pub fn parse(stream: &Stream) -> Result<File, ilex::Fatal> {
    let mut cursor = stream.cursor();
    let ctx = &cursor.context();

    let crust = Crust::get();
    let mut file: Vec<Declaration> = vec![];
    let report = ctx.new_report();

    // where import and module parsing would go
    while !cursor.is_empty() {
        // Find and parse next item, and add to vec
        let decl = parse_declaration(&mut cursor, ctx, crust, &report);
        if let Some(decl) = decl {
            file.push(decl);
        }
    }

    Ok(File(file))
}

// pub enum Declaration {
//     TypeDefinition(TypeDefinition),
//     FunctionDeclaration(FunctionDeclaration),
//     FunctionDefinition(FunctionDefinition),
//     StaticVariableDeclaration(StaticVariableDeclaration)
// }
fn parse_declaration(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
) -> Option<Declaration> {
    let decl = token::switch()
        .case(crust.static_, |_, cursor| {
            // static token has been parsed, can be ignored
            let declaration = parse_variable_declaration(cursor, ctx, crust, report);
            Some(Declaration::StaticVariableDeclaration(
                StaticVariableDeclaration { declaration },
            ))
        })
        .cases([crust.struct_, crust.enum_, crust.union], |tok, cursor| {
            // back up so we can read type again
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);

            // Next is either brackets, containing type information, or an ident, for a function
            // name.

            if let Some(_) = cursor.peek(crust.ident) {
                return Some(parse_function(cursor, ctx, crust, report, ty))
            }

            // if it is a valid "struct", "ident" format, then we need to back up one, so the ident
            // becomes available
            cursor.back_up(1);
            let type_decl = parse_type_definition(cursor, ctx, crust, report, tok);
            Some(Declaration::TypeDefinition(type_decl))
        })
        .case(crust.typedef, |_, cursor| {
            Some(Declaration::TypeDefinition(parse_typedef(
                cursor, ctx, crust, report,
            )))
        })
        .case(crust.ident, |tok, cursor| {
            // the only way this happens is by a typedef'd user type being the return value of a
            // function
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);
            Some(parse_function(cursor, ctx, crust, report, ty))
        })
        .case(crust.parens, |tok, cursor| {
            // if this happens, probably returns a tuple. Parse it, then return the valid function
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);
            Some(parse_function(cursor, ctx, crust, report, ty))
        })
        // Going to match any compiler types
        .cases(
            [
                crust.u8,
                crust.u16,
                crust.u32,
                crust.u64,
                crust.usize,
                crust.i8,
                crust.i16,
                crust.i32,
                crust.i64,
                crust.isize,
                crust.z8,
                crust.z16,
                crust.z32,
                crust.z64,
                crust.zsize,
                crust.f32,
                crust.f64,
                crust.c32,
                crust.c64,
                crust.bool,
            ],
            |_, cursor| {
                cursor.back_up(1);
                let ty = parse_type(cursor, ctx, crust, report);
                Some(parse_function(cursor, ctx, crust, report, ty))
            },
        )
        .case(Lexeme::eof(), |_, _| None)
        .take(cursor, report)??;

    Some(decl)
}

// pub struct StaticVariableDeclaration {
//     pub declaration: VariableDeclaration
// }
fn parse_variable_declaration(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
) -> VariableDeclaration {
    todo!()
}

// pub enum TypeDefinition {
//     StructDef {
//         ident: String,
//         fields: Vec<(String, Type)>,
//     },
//     UnionDef {
//         ident: String,
//         fields: Vec<(String, Type)>,
//     },
//     EnumDef {
//         ident: String,
//         values: Vec<(String, Option<Type>)>,
//     },
//     ...
// }
fn parse_type_definition(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
    token: token::Keyword,
) -> TypeDefinition {
    todo!()
}

// Can return either a FunctionDefinition or a FunctionDefinition
// NOTE(Declaration):
// pub struct FunctionDeclaration {
//     return_type: Type,
//     ident: String,
//     arguments: Vec<(Type, String)>
// }
// NOTE(Definition):
// pub struct FunctionDefinition {
//     declaration: FunctionDeclaration,
//     code: CodeBlock
// }
fn parse_function(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
    ret: Type
) -> Declaration {
    todo!()
}

// Returns a typedef
// pub enum TypeDefinition {
//     ...
//     TypeDef {
//         old_type: Box<Type>,
//         alias: String,
//     },
// }
fn parse_typedef(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
) -> TypeDefinition {
    // typedef can be ignored, that token has been consumed
    todo!()
}

// pub enum Type {
//     U8,
//     U16,
//     U32,
//     U64,
//     USize,
//     I8,
//     I16,
//     I32,
//     I64,
//     ISize,
//     Z8,
//     Z16,
//     Z32,
//     Z64,
//     ZSize,
//     F32,
//     F64,
//     C32,
//     C64,
//     Bool,
//     Void,
//     // tuple-def = "(", [ type-specifier ], { ",", type-specifier }, ")", ";" ;
//     Tuple(Vec<Type>),
//     // array-def = type-specifier, [ "[", number, "]", ], { "[", number, "]", };
//     Array(Box<Type>, Vec<usize>),
//     Struct(String),
//     Enum(String),
//     Union(String),
//     Pointer(Box<Type>),
// }
fn parse_type(cursor: &mut Cursor, ctx: &Context, crust: &Crust, report: &Report) -> Type {
    todo!()
}
