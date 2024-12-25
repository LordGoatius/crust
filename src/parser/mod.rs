pub mod ast;
pub mod ops;

use ast::*;
use ilex::token::Stream;

use crate::lexer::Crust;

pub fn parse(stream: &Stream) -> Result<File, ilex::Fatal> {
    let mut cursor = stream.cursor();

    let crust = Crust::get();
    
    // where import and module parsing would go

    todo!()
}

