pub mod ast;
pub mod ops;

use std::cell::RefCell;

use ast::*;
use ilex::{
    token::{self, Cursor, Stream},
    Context, Lexeme, Report, Spanned,
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
        // Static
        .case(crust.static_, |_, cursor| {
            // static token has been parsed, can be ignored
            let declaration = parse_variable_declaration(cursor, ctx, crust, report);
            Some(Declaration::StaticVariableDeclaration(
                StaticVariableDeclaration { declaration },
            ))
        })
        // Type Definition OR Function Signature
        .cases([crust.struct_, crust.enum_, crust.union], |_, cursor| {
            // back up so we can read type again
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);

            // Next is either brackets, containing type information, or an ident, for a function
            // name.

            if let Some(_) = cursor.peek(crust.ident) {
                return Some(parse_function(cursor, ctx, crust, report, ty));
            }

            // if it is a valid "struct", "ident" format, then we need to back up one, so the ident
            // becomes available
            cursor.back_up(1);
            let type_decl = parse_type_definition(cursor, ctx, crust, report, ty);
            Some(Declaration::TypeDefinition(type_decl))
        })
        // Type Definition
        .case(crust.typedef, |_, cursor| {
            Some(Declaration::TypeDefinition(parse_typedef(
                cursor, ctx, crust, report,
            )))
        })
        // Function Signature
        .case(crust.ident, |_, cursor| {
            // the only way this happens is by a typedef'd user type being the return value of a
            // function
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);
            Some(parse_function(cursor, ctx, crust, report, ty))
        })
        // Function Signature
        .case(crust.parens, |_, cursor| {
            // if this happens, probably returns a tuple. Parse it, then return the valid function
            cursor.back_up(1);
            let ty = parse_type(cursor, ctx, crust, report);
            Some(parse_function(cursor, ctx, crust, report, ty))
        })
        // Function Signature
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
                crust.void,
            ],
            |_, cursor| {
                cursor.back_up(1);
                let ty = parse_type(cursor, ctx, crust, report);
                Some(parse_function(cursor, ctx, crust, report, ty))
            },
        )
        // End
        .case(Lexeme::eof(), |_, _| None)
        .take(cursor, report)
        .expect("This shouldn't fail until error handling is upgraded after codegen works")?;

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
    ty: Type,
) -> TypeDefinition {
    // starts at the brackets (ideally)
    match ty {
        Type::Struct(ident) => parse_struct_def(cursor, ctx, crust, report, ident),
        Type::Union(ident) => parse_union_def(cursor, ctx, crust, report, ident),
        Type::Enum(ident) => parse_enum_def(cursor, ctx, crust, report, ident),
        _ => unreachable!(),
    }
}

// type-definition = (( "struct", ident, "{", { ident, type-specifier, ";" }, "}" )
fn parse_struct_def(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
    ident: String,
) -> TypeDefinition {
    let fields = cursor
        .take(crust.block, report)
        .unwrap()
        .contents()
        .delimited(crust.semicolon, |tokens| {
            let ident = tokens.take(crust.ident, report).unwrap();
            let type_spec = parse_type(tokens, ctx, crust, report);
            Some((ident.text(ctx).to_string(), type_spec))
        })
        .map(|x| x.0)
        .collect();

    TypeDefinition::StructDef { ident, fields }
}

// | ( "enum",   ident, "{", { ident, type-specifier, "," }, "}" )
fn parse_enum_def(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
    ident: String,
) -> TypeDefinition {
    let values = cursor
        .take(crust.block, report)
        .unwrap()
        .contents()
        .delimited(crust.comma, |tokens| {
            let ident = tokens.take(crust.ident, report).unwrap();
            let type_opt = match tokens.try_take(crust.comma) {
                None => Some(parse_type(cursor, ctx, crust, report)),
                Some(_) => None,
            };
            Some((ident.text(ctx).to_string(), type_opt))
        })
        .map(|x| x.0)
        .collect();

    TypeDefinition::EnumDef { ident, values }
}

// | ( "union",  ident, "{", { ident, type-specifier, ";" }, "}" )
fn parse_union_def(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
    ident: String,
) -> TypeDefinition {
    // This will stay TODO lol
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
    ret: Type,
) -> Declaration {
    let ident = cursor.take(crust.ident, report).unwrap();

    let arguments = cursor
        .take(crust.parens, report)
        .unwrap()
        .contents()
        .delimited(crust.comma, |tokens| {
            let arg_type = parse_type(tokens, ctx, crust, report);
            let arg_ident = tokens.take(crust.ident, report).unwrap();
            Some((arg_type, arg_ident.text(ctx).to_owned()))
        })
        .map(|x| x.0)
        .collect();

    let declaration = FunctionDeclaration {
        return_type: ret,
        ident: ident.text(ctx).to_string(),
        arguments,
    };

    if let Some(_) = cursor.try_take(crust.semicolon) {
        return Declaration::FunctionDeclaration(declaration);
    }

    let code = parse_code_block(cursor, ctx, crust, report);

    Declaration::FunctionDefinition(FunctionDefinition { declaration, code })
}

fn parse_code_block(
    cursor: &mut Cursor,
    ctx: &Context,
    crust: &Crust,
    report: &Report,
) -> CodeBlock {
    todo!()
}

/// NOTE: Typedef does not support C style struct typedefs as such:
/// ```c
/// typedef struct {
///    ...
/// } my_struct;
/// ```
/// It must be:
/// ```c
/// struct my_struct {
///     ...
/// };
/// typedef my_struct my_struct;
/// ```
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
    let prev = parse_type(cursor, ctx, crust, report);
    let ident = cursor.take(crust.ident, report).unwrap();

    TypeDefinition::TypeDef {
        old_type: Box::new(prev),
        alias: ident.text(ctx).to_string(),
    }
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
    let ty = token::switch()
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
                crust.void,
            ],
            // Have to check pointer OR array
            |tok, _| match tok.text(ctx) {
                "u8" => Type::U8,
                "u16" => Type::U16,
                "u32" => Type::U32,
                "u64" => Type::U64,
                "usize" => Type::USize,
                "i8" => Type::I8,
                "i16" => Type::I16,
                "i32" => Type::I32,
                "i64" => Type::I64,
                "isize" => Type::ISize,
                "z8" => Type::Z8,
                "z16" => Type::Z16,
                "z32" => Type::Z32,
                "z64" => Type::Z64,
                "zsize" => Type::ZSize,
                "f32" => Type::F32,
                "f64" => Type::F64,
                "c32" => Type::C32,
                "c64" => Type::C64,
                "bool" => Type::Bool,
                "void" => Type::Void,
                _ => unreachable!(),
            },
        )
        // Tuple
        .case(crust.parens, |tok, _| {
            Type::Tuple(
                tok.contents()
                    .delimited(crust.comma, |tokens| {
                        // Toks is a cursor that contains the comma delimited tokens in the
                        // parentheses around our tuple.
                        // This means each object separated by parentheses must be a valid type, and
                        // only a valid type.
                        // We can then recursively call this function on each, adding them to a Vec.
                        // Eventually, we'll end in a terminal type, and return.
                        let ty = parse_type(tokens, ctx, crust, report);
                        Some(ty)
                    })
                    .map(|(i, _)| i)
                    .collect(),
            )
        })
        .case(crust.struct_, |_, cursor| {
            if let Some(ident) = cursor.try_take(crust.ident) {
                // ugh don't wanna deal with lifetimes I'll refactor it later don't @ me
                Type::Struct(ident.text(ctx).to_string())
            } else {
                panic!()
            }
        })
        .case(crust.enum_, |_, cursor| {
            if let Some(ident) = cursor.try_take(crust.ident) {
                // ugh don't wanna deal with lifetimes I'll refactor it later don't @ me
                Type::Enum(ident.text(ctx).to_string())
            } else {
                panic!()
            }
        })
        .case(crust.union, |_, cursor| {
            if let Some(ident) = cursor.try_take(crust.ident) {
                // ugh don't wanna deal with lifetimes I'll refactor it later don't @ me
                Type::Union(ident.text(ctx).to_string())
            } else {
                panic!()
            }
        })
        .take(cursor, report)
        .unwrap_or_else(|| unreachable!(":)"));

    let ty_cell = RefCell::new(ty);

    // Rust hates me :(
    // It's safe, just trust me
    // What if I wrapped it in an unsafecell
    // NOTE: Line (curr - 5)
    #[rustfmt::skip]
    while let Some(wrapped) = token::switch()
        .case(crust.array, |tok, _| {
            let length: Vec<_> = tok.contents().collect();
            if length.len() > 1 || length.len() == 0 {
                panic!(":( not valid array len")
            }

            let len = token::switch()
                .case(crust.integer_literal, |tok, _| {
                    Some(str::parse::<usize>(tok.text(ctx)).unwrap_or_else(|_| unreachable!()))
                })
                .case(crust.hex_literal, |tok, _| {
                    Some(
                        usize::from_str_radix(&tok.text(ctx)[2..], 16)
                            .unwrap_or_else(|_| unreachable!()),
                    )
                })
                .case(crust.octal_literal, |tok, _| {
                    Some(
                        usize::from_str_radix(&tok.text(ctx)[2..], 8)
                            .unwrap_or_else(|_| unreachable!()),
                    )
                })
                .case(crust.binary_literal, |tok, _| {
                    Some(
                        usize::from_str_radix(&tok.text(ctx)[2..], 2)
                            .unwrap_or_else(|_| unreachable!()),
                    )
                })
                .take(&mut tok.contents(), report)
                .expect("Oops! all integers 0")
                .expect("Oops! all integers 1");

            let ty_clone = ty_cell.borrow().clone();
            Type::Array(Box::new(ty_clone), len)
        })
        .case(crust.star, |_, _| {
            let ty_clone = ty_cell.borrow().clone();
            Type::Pointer(Box::new(ty_clone))
        })
        .try_take(cursor)
    {
        *ty_cell.borrow_mut() = wrapped;
    }

    ty_cell.into_inner()
}
