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
    let str =
"
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

    let ast1= grammar::CustomTypeDefinitionParser::new()
        .parse("typedef struct mystr** pmystr");
    let ast2= grammar::CustomTypeDefinitionParser::new()
        .parse("struct structype { my_int i32; my_fl f64; }");
    let num = grammar::NumberParser::new()
        .parse("12_34");
    let arr = grammar::TyParser::new()
        .parse("struct mything[4][5][7]");

    let file= grammar::FileParser::new()
        .parse(str);

    println!("{ast1:?}");
    println!("{ast2:?}");
    println!("{num:?}");
    println!("{arr:?}");
    println!("{file:#?}");
    return;
    // let file = std::env::args().nth(1).unwrap();

    // let ctx = ilex::Context::new();
    // let report = ctx.new_report();

    // let file = ctx.open_file(file, &report).unwrap();
    // let tokens = file.lex(Crust::get().spec(), &report).unwrap();

    // // print_tree(tokens.cursor(), &ctx, 0);
    // let tree = parse(&tokens);
    // println!("{tree:?}");
    // // let cursor = tokens.cursor();
}

// fn print_tree(tokens: Cursor, ctx: &ilex::Context, val: u8) {
//     for token in tokens {
//         if let Ok(br) = token.bracket() {
//             for _ in 0..val {
//                 print!("\t");
//             }
//             print!("{:?}", br.open().text(ctx).fg::<Gray>());
//             println!();
//             print_tree(br.contents(), ctx, val + 1);

//             for _ in 0..val {
//                 print!("\t");
//             }
//             print!("{:?}", br.close().text(ctx).fg::<Gray>());
//             println!();
//         } else {
//             for _ in 0..val {
//                 print!("\t");
//             }
//             match token {
//                 ilex::token::Any::Keyword(token) => println!("{:?}, {:?}", token.text(ctx).fg::<Gold>(), token.fg::<Green>()),
//                 ilex::token::Any::Ident(token) => println!("{:?}, {:?}", token.text(ctx).fg::<Blue>(), token.fg::<Green>()),
//                 ilex::token::Any::Digital(token) => println!("{:?}, {:?}", token.text(ctx).fg::<Red>(), token.fg::<Green>()),
//                 ilex::token::Any::Quoted(token) => println!("{:?}, {:?}", token.text(ctx).fg::<Brown>(), token.fg::<Green>()),
//                 _ => (),
//             }
//         }
//     }
// }
