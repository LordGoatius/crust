#![feature(stmt_expr_attributes)]
// pub mod lexer;
pub mod parser;

// use ilex::token::Cursor;
// use ilex::Spanned;
use lalrpop_util::lalrpop_mod;
// use owo_colors::{colors::{css::{Brown, Gold, Gray}, Blue, Green, Red}, OwoColorize};
// use parser::parse;

lalrpop_mod!(pub grammar);

// use crate::lexer::Crust;

fn main() {
    let str = "
struct structype { my_int i32;
    my_fl f64;
};
enum int_opt {
    some: i32,
    left: struct ident*,
    none,
};
typedef struct structype** ppstrty;
";

    let ast1 = grammar::CustomTypeDefinitionParser::new().parse("typedef struct mystr** pmystr");
    let ast2 = grammar::CustomTypeDefinitionParser::new()
        .parse("struct structype { my_int i32; my_fl f64; }");
    let num = grammar::NumberParser::new().parse("12_34");
    let arr = grammar::TyParser::new().parse("struct mything[4][5][7]");

    let file = grammar::FileParser::new().parse(str);

    let file2 = grammar::FileParser::new().parse(
        "static i64[7] thing = [1, 2, 4, 5];
             static (i32, usize) thing;
             static (struct tup, le) name = (7; 6; 13);",
    );

    ast1.unwrap();
    ast2.unwrap();
    num.unwrap();
    arr.unwrap();
    file.unwrap();
    file2.unwrap();
    return;
}
