#![allow(unused)]

use ast::*;
use ilex::{rule::{Any, *}, token::{self, Any as TokenAny, Cursor}, Context, Lexeme, Spanned};
pub mod ast;
pub mod ops;

pub struct ParseError;

pub fn parse_program(tokens: &mut Cursor, ctx: &Context) -> Result<TranslationUnit, ParseError> {
    let prog: Vec<Declaration> = vec![];
    while let None = tokens.peek(Lexeme::<Eof>::z()) {
        let declaration = try_parse_declaration(tokens, ctx)?;
    }
    Ok(TranslationUnit(prog))
}

// pub enum Declaration {
//     TypeDefinition(TypeDefinition),
//     FunctionDeclaration(FunctionDeclaration),
//     FunctionDefinition(FunctionDefinition),
//     StaticVariableDeclaration(StaticVariableDeclaration)
// }
fn try_parse_declaration(tokens: &mut Cursor, ctx: &Context) -> Result<Declaration, ParseError> {
    if let Ok(type_definition) = try_parse_type_definition(tokens, ctx) {
        return Ok(Declaration::TypeDefinition(type_definition));
    }

    if let Ok(function_definition) = try_parse_function_definition(tokens, ctx) {
        return Ok(Declaration::FunctionDefinition(function_definition));
    }

    if let Ok(function_declaration) = try_parse_function_declaration(tokens, ctx) {
        return Ok(Declaration::FunctionDeclaration(function_declaration));
    }

    if let Ok(static_var_declaration) = try_parse_static_variable_declaration(tokens, ctx) {
        return Ok(Declaration::StaticVariableDeclaration(static_var_declaration));
    }

    Err(ParseError)
}

// pub struct StaticVariableDeclaration {
//     pub declaration: VariableDeclaration
// }
fn try_parse_static_variable_declaration(tokens: &mut Cursor, ctx: &Context) -> Result<StaticVariableDeclaration, ParseError> {
    if let Some("static") = tokens.peek(Lexeme::<Keyword>::z()).map(|t| t.text(ctx)) {
        let declaration = try_parse_variable_declaration(tokens, ctx)?;
        let _ = tokens.next(); // Static is only valid token in this context
        Ok(StaticVariableDeclaration { declaration })
    } else {
        Err(ParseError)
    }
}

// variable-declaration = 
//     type-specifier, ident [ "=", type-instantiation ], { ",", ident, [ "=", type-instantiation ] }, ";" ;
//
// pub struct VariableDeclaration {
//     var_type: Type, 
//     ident: String,
//     definition: Option<TypeInstantiation>,
//     extra: Vec<(String, Option<TypeInstantiation>)>
fn try_parse_variable_declaration(tokens: &mut Cursor, ctx: &Context) -> Result<VariableDeclaration, ParseError> {
    let var_type = try_parse_type(tokens, ctx)?;
    // next token must be identifier
    if let Some(TokenAny::Ident(ident)) = tokens.next() {
        if let Some(";") = tokens.peek_any().map(|t| t.text(ctx)) {
            let ident = ident.text(ctx).into();
            return Ok(VariableDeclaration { var_type, ident, definition: None, extra: vec![] });
        }
        // TODO: "=" and type-instantiation are optional, figure out if they're there or not.
        // Then write `try_parse_declaration_list` and return the vec of
        if let Some("=") = tokens.next().map(|t| t.text(ctx)) {
            todo!()
        } else {
            todo!()
        }
    } else {
        Err(ParseError)
    }
}

fn try_parse_type(tokens: &mut Cursor, ctx: &Context) -> Result<Type, ParseError> {
    todo!()
}



