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

    let arr = grammar::TyParser::new().parse("struct mything[4][5][7]");

    let file = grammar::FileParser::new().parse(str);

    let file2 = grammar::FileParser::new().parse(
        "static i64[7] thing = [1, 2, 4, 5];
         static (i32, usize) thing;
         static (struct tup, le) name = <7; 6; 13>;",
    );

    let file3 = grammar::FileParser::new().parse(
        "static usize thing = (17 ({ident}($ident, $ident2) $twelve -) +);"
    );

    arr.unwrap();
    file.unwrap();
    file2.unwrap();
    dbg!(file3.unwrap());
    return;
}
