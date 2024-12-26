pub mod ast;
pub mod ops;

use ast::*;
use ilex::{token::{self, Cursor, Stream}, Context, Lexeme, Report};

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
        let decl = parse_declaration(&mut cursor, ctx, crust, &report)?;
        file.push(decl);
    }

    Ok(File(file))
}

// pub enum Declaration {
//     TypeDefinition(TypeDefinition),
//     FunctionDeclaration(FunctionDeclaration),
//     FunctionDefinition(FunctionDefinition),
//     StaticVariableDeclaration(StaticVariableDeclaration)
// }
fn parse_declaration(cursor: &mut Cursor, ctx: &Context, crust: &Crust, report: &Report) -> Result<Declaration, ilex::Fatal> {
    let decl = token::switch()
        .case(crust.static_, |tok, cursor| {
            // parse static
            let declaration = todo!(); // parse_variable_declaration(...);
            Some(Declaration::StaticVariableDeclaration(StaticVariableDeclaration { declaration }))
        })
        .case(crust.struct_, |tok, cursor| todo!())
        .case(crust.enum_,   |tok, cursor| todo!())
        .case(crust.union,   |tok, cursor| todo!())
        .case(Lexeme::eof(), |_, _| { None })
        .take(cursor, report).unwrap().unwrap();

    Ok(decl)
}
