#![allow(unused)]

use ast::*;
use ilex::{rule::{Any, *}, token::{self, Any as TokenAny, Cursor}, Context, Lexeme, Spanned};
pub mod ast;
pub mod ops;

pub struct ParseError();

pub fn parse_program(tokens: &Cursor, ctx: &Context) -> Result<TranslationUnit, ParseError> {
    let prog: Vec<Declaration> = vec![];
    while let None = tokens.peek(Lexeme::<Eof>::z()) {
        let declaration = try_parse_declaration(tokens, ctx)?;
    }
    Ok(TranslationUnit(prog))
}

fn try_parse_declaration(tokens: &Cursor, ctx: &Context) -> Result<Declaration, ParseError> {
    todo!()
}

//pub fn parse(tokens: &mut Cursor, ctx: &Context) -> TranslationUnit {
//    let mut program: Vec<Declaration> = vec![];
//    let next = match tokens.peek_any() {
//        Some(TokenAny::Keyword(keyword)) => {
//            match keyword.text(ctx) {
//                "static" => {
//                    let _ = tokens.next();
//                    let declaration = parse_variable_declaration(tokens, ctx);
//                    Declaration::StaticVariableDeclaration(StaticVariableDeclaration { declaration })
//                },
//                "typedef" => {
//                    let typedef = parse_typedef(tokens, ctx);
//                    Declaration::TypeDefinition(typedef)
//                },
//                // handle struct (which can be function or struct definition)
//                "struct" => {
//                    // bracket - struct def 
//                    // ident   - function return type
//                    match tokens.nth(2) {
//
//                        _ => todo!(),
//                    }
//                    todo!()
//                },
//                _ => todo!()
//            }
//        },
//        Some(TokenAny::Ident(ident)) => {
//            // user defined type?
//            todo!()
//        },
//        None | Some(TokenAny::Eof(_)) => return TranslationUnit(program),
//        Some(_) => panic!("static out-of-function variables only"),
//    };
//    todo!()
//}
//
//fn parse_variable_declaration(tokens: &mut Cursor, ctx: &Context) -> VariableDeclaration {
//    todo!()
//}
//
//fn parse_typedef(tokens: &mut Cursor, ctx: &Context) -> TypeDefinition {
//    todo!()
//}
