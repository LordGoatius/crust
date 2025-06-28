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
    struct structype {
        my_int i32;
        my_fl f64;
    };
    enum int_opt {
        some: i32,
        left: struct ident*,
        none,
    };
    typedef struct structype** ppstrty;

    static i64[7] thing = [1, 2, 4, 5];
    static <i32, usize> thing;
    static <struct tup, le> name = <7; 6; 13>;

    u32 name(struct ident arg1, u32 thing);
    u32 name(struct ident arg1, u32 thing) {}";
    let file = grammar::FileParser::new().parse(str);

    dbg!(file.unwrap());
    return;
}
