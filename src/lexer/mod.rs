use ilex::rule::*;
use ilex::Lexeme;

#[ilex::spec]
pub struct Crust {
    // #[rule(Ident::new()
    //     .ascii_only()
    //     .extra_continue('.'))]
    // ident_access: Lexeme<Ident>,

    #[rule("//")]
    pub comment: Lexeme<Comment>,

    #[rule("/*", "*/")]
    pub block_comment: Lexeme<Comment>,

    #[rule(Digital::new(16)
        .separator('_')
        .minus()
        .prefix("0x")
        )]
    pub hex_literal: Lexeme<Digital>,

    #[rule(Digital::new(8)
        .separator('_')
        .minus()
        .prefix("0o")
        )]
    pub octal_literal: Lexeme<Digital>,

    #[rule(Digital::new(2)
        .separator('_')
        .minus()
        .prefix("0b")
        )]
    pub binary_literal: Lexeme<Digital>,

    #[rule(Digital::new(10)
        .separator('_')
        .minus()
        )]
    pub integer_literal: Lexeme<Digital>,

    #[named]
    #[rule(Quoted::new('"')
        .invalid_escape(r"\")
        .escapes([
            "\\\"", r"\\", r"\/",
            r"\b", r"\f",  r"\n", r"\t", r"\r",
        ]))]
    pub string: Lexeme<Quoted>,

    #[named]
    #[rule(Quoted::new('\'')
        .escapes([
            r"\b", r"\f",  r"\n", r"\t", r"\r",
        ])
        )]
    pub char: Lexeme<Quoted>,

    #[named]
    #[rule("[", "]")]
    pub array: Lexeme<Bracket>,

    #[named]
    #[rule("{", "}")]
    pub block: Lexeme<Bracket>,

    #[named]
    #[rule("(", ")")]
    pub parens: Lexeme<Bracket>,

    #[rule('_')]
    unused: Lexeme<Keyword>,

    #[named]
    #[rule(".")]
    pub dot: Lexeme<Keyword>,

    #[rule('*')]
    pub star: Lexeme<Keyword>,

    #[rule("->")]
    pub thin_arrow: Lexeme<Keyword>,

    #[rule("++")]
    pub plusplus: Lexeme<Keyword>,

    #[rule("--")]
    pub minusminus: Lexeme<Keyword>,

    #[rule('+')]
    pub add: Lexeme<Keyword>,

    #[rule('-')]
    pub neg: Lexeme<Keyword>,

    #[rule('/')]
    pub div: Lexeme<Keyword>,

    #[rule('%')]
    pub mod_op: Lexeme<Keyword>,

    #[rule('&')]
    pub and: Lexeme<Keyword>,

    #[rule("|")]
    pub or: Lexeme<Keyword>,

    #[rule("^")]
    pub xor: Lexeme<Keyword>,

    #[rule("~")]
    pub not: Lexeme<Keyword>,

    #[rule("!")]
    pub log_not: Lexeme<Keyword>,

    #[rule("&&")]
    pub log_and: Lexeme<Keyword>,

    #[rule("||")]
    pub log_or: Lexeme<Keyword>,

    #[rule("?")]
    pub tern: Lexeme<Keyword>,

    #[rule(":")]
    pub colon: Lexeme<Keyword>,

    #[rule(">>")]
    pub shr: Lexeme<Keyword>,

    #[rule("<<")]
    pub shl: Lexeme<Keyword>,

    #[rule("<")]
    pub less: Lexeme<Keyword>,

    #[rule(">")]
    pub greater: Lexeme<Keyword>,

    #[rule("<=")]
    pub leq: Lexeme<Keyword>,

    #[rule(">=")]
    pub geq: Lexeme<Keyword>,

    #[rule("==")]
    pub equals: Lexeme<Keyword>,

    #[rule("!=")]
    pub not_eq: Lexeme<Keyword>,

    #[rule(";")]
    pub semicolon: Lexeme<Keyword>,

    #[rule(",")]
    pub comma: Lexeme<Keyword>,

    #[rule("=>")]
    pub fat_arrow: Lexeme<Keyword>,

    #[rule("union")]
    pub union: Lexeme<Keyword>,

    #[rule("typedef")]
    pub typedef: Lexeme<Keyword>,

    #[rule("sizeof")]
    pub sizeof: Lexeme<Keyword>,

    #[rule("mod")]
    pub mod_: Lexeme<Keyword>,

    #[rule("struct")]
    pub struct_: Lexeme<Keyword>,

    #[rule("enum")]
    pub enum_: Lexeme<Keyword>,

    #[rule("for")]
    pub for_: Lexeme<Keyword>,

    #[rule("while")]
    pub while_: Lexeme<Keyword>,

    #[rule("if")]
    pub if_: Lexeme<Keyword>,

    #[rule("else")]
    pub else_: Lexeme<Keyword>,

    #[rule("return")]
    pub return_: Lexeme<Keyword>,

    #[rule("static")]
    pub static_: Lexeme<Keyword>,

    #[rule("match")]
    match_: Lexeme<Keyword>,

    #[rule("let")]
    pub let_: Lexeme<Keyword>,

    #[rule("u8")]
    pub u8: Lexeme<Keyword>,
    #[rule("u16")]
    pub u16: Lexeme<Keyword>,
    #[rule("u32")]
    pub u32: Lexeme<Keyword>,
    #[rule("u64")]
    pub u64: Lexeme<Keyword>,
    #[rule("usize")]
    pub usize: Lexeme<Keyword>,
    #[rule("i8")]
    pub i8: Lexeme<Keyword>,
    #[rule("i16")]
    pub i16: Lexeme<Keyword>,
    #[rule("i32")]
    pub i32: Lexeme<Keyword>,
    #[rule("i64")]
    pub i64: Lexeme<Keyword>,
    #[rule("isize")]
    pub isize: Lexeme<Keyword>,
    #[rule("z8")]
    pub z8: Lexeme<Keyword>,
    #[rule("z16")]
    pub z16: Lexeme<Keyword>,
    #[rule("z32")]
    pub z32: Lexeme<Keyword>,
    #[rule("z64")]
    pub z64: Lexeme<Keyword>,
    #[rule("zsize")]
    pub zsize: Lexeme<Keyword>,
    #[rule("f32")]
    pub f32: Lexeme<Keyword>,
    #[rule("f64")]
    pub f64: Lexeme<Keyword>,
    #[rule("c32")]
    pub c32: Lexeme<Keyword>,
    #[rule("c64")]
    pub c64: Lexeme<Keyword>,
    #[rule("bool")]
    pub bool: Lexeme<Keyword>,
    #[rule("void")]
    pub void: Lexeme<Keyword>,

    #[rule("=")]
    pub eq: Lexeme<Keyword>,
    #[rule("+=")]
    pub add_eq: Lexeme<Keyword>,
    #[rule("-=")]
    pub sub_eq: Lexeme<Keyword>,
    #[rule("*=")]
    pub mul_eq: Lexeme<Keyword>,
    #[rule("\\=")]
    pub div_eq: Lexeme<Keyword>,
    #[rule("%=")]
    pub mod_eq: Lexeme<Keyword>,
    #[rule(">>=")]
    pub shr_eq: Lexeme<Keyword>,
    #[rule("<<=")]
    pub shl_eq: Lexeme<Keyword>,
    #[rule("&=")]
    pub and_eq: Lexeme<Keyword>,
    #[rule("^=")]
    pub xor_eq: Lexeme<Keyword>,
    #[rule("|=")]
    pub or_eq: Lexeme<Keyword>,

    #[named("ident")]
    #[rule(Ident::new()
        .ascii_only()
        .prefixes('a'..='z')
        .prefixes('A'..='Z')
        .prefixes('_'..='_')
        )]
    pub ident: Lexeme<Ident>,
    // #[named]
    // #[rule(Digital::new(10)
    //     .separator('_')
    //     .minus()
    //     .point_limit(0..2)
    //     .exponents(["e", "E"], Digits::new(10).plus().minus()))]
    // float_literal: Lexeme<Digital>,
}
