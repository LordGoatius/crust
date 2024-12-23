pub mod lexer;
pub mod parser;

use ilex::token::Cursor;
use ilex::Spanned;

use crate::lexer::Crust;

fn main() {
    let file = std::env::args().nth(1).unwrap();

    let ctx = ilex::Context::new();
    let report = ctx.new_report();

    let file = ctx.open_file(file, &report).unwrap();
    let tokens = file.lex(Crust::get().spec(), &report).unwrap();

    print_tree(tokens.cursor(), &ctx, 0);
    // let cursor = tokens.cursor();
}

fn print_tree(tokens: Cursor, ctx: &ilex::Context, val: u8) {
    for token in tokens {
        if let Ok(br) = token.bracket() {
            for _ in 0..val {
                print!("\t");
            }
            print!("{:?}", br.open().text(ctx));
            println!();
            print_tree(br.contents(), ctx, val + 1);

            for _ in 0..val {
                print!("\t");
            }
            print!("{:?}", br.close().text(ctx));
            println!();
        } else {
            for _ in 0..val {
                print!("\t");
            }
            println!("{:?}", token.text(ctx));
        }
    }
}
