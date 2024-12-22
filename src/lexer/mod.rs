use ilex::rule::*;
use ilex::Lexeme;

#[ilex::spec]
pub struct Crust {
    #[named("ident")]
    #[rule(Ident::new()
        .ascii_only())]
    ident: Lexeme<Ident>,

    #[named("ident_access")]
    #[rule(Ident::new()
        .ascii_only()
        .extra_continue('.'))]
    ident_access: Lexeme<Ident>,

    #[rule("//")]
    comment: Lexeme<Comment>,

    #[rule(Digital::new(10)
        .separator('_')
        .minus()
        )]
    integer_literal: Lexeme<Digital>,

    #[rule(Digital::new(16)
        .separator('_')
        .minus()
        .prefix("0x")
        )]
    hex_literal: Lexeme<Digital>,

    #[rule(Digital::new(2)
        .separator('_')
        .minus()
        .prefix("0b")
        )]
    bin_literal: Lexeme<Digital>,

    #[named]
    #[rule(Digital::new(10)
        .minus()
        .point_limit(0..2)
        .exponents(["e", "E"], Digits::new(10).plus().minus()))]
    float_literal: Lexeme<Digital>,

    #[named]
    #[rule(Quoted::new('"')
        .invalid_escape(r"\")
        .escapes([
            "\\\"", r"\\", r"\/",
            r"\b", r"\f",  r"\n", r"\t", r"\r",
        ]))]
    string: Lexeme<Quoted>,

    #[named]
    #[rule(Quoted::new('\'')
        .invalid_escape(r"\")
        .escapes([
            r"\\", r"\/",
            r"\b", r"\f",  r"\n", r"\t", r"\r",
        ]))]
    char: Lexeme<Quoted>,

    #[named]
    #[rule("[", "]")]
    array: Lexeme<Bracket>,

    #[named]
    #[rule("{", "}")]
    block: Lexeme<Bracket>,

    #[named]
    #[rule("(", ")")]
    parens: Lexeme<Bracket>,

    #[rule('_')]
    unused: Lexeme<Keyword>,

    #[rule('*')]
    star: Lexeme<Keyword>,

    #[rule('.')]
    dot: Lexeme<Keyword>,

    #[rule("->")]
    thin_arrow: Lexeme<Keyword>,

    #[rule("++")]
    plusplus: Lexeme<Keyword>,

    #[rule("--")]
    minusminus: Lexeme<Keyword>,

    #[rule('+')]
    add: Lexeme<Keyword>,

    #[rule('-')]
    neg: Lexeme<Keyword>,

    #[rule('/')]
    div: Lexeme<Keyword>,

    #[rule('%')]
    mod_op: Lexeme<Keyword>,

    #[rule('&')]
    and: Lexeme<Keyword>,

    #[rule("|")]
    or: Lexeme<Keyword>,

    #[rule("^")]
    xor: Lexeme<Keyword>,

    #[rule("~")]
    not: Lexeme<Keyword>,

    #[rule("!")]
    log_not: Lexeme<Keyword>,

    #[rule("&&")]
    log_and: Lexeme<Keyword>,

    #[rule("||")]
    log_or: Lexeme<Keyword>,

    #[rule("?")]
    tern: Lexeme<Keyword>,

    #[rule(":")]
    colon: Lexeme<Keyword>,

    #[rule(">>")]
    shr: Lexeme<Keyword>,

    #[rule("<<")]
    shl: Lexeme<Keyword>,

    #[rule("<")]
    less: Lexeme<Keyword>,

    #[rule(">")]
    greater: Lexeme<Keyword>,

    #[rule("<=")]
    leq: Lexeme<Keyword>,

    #[rule(">=")]
    geq: Lexeme<Keyword>,

    #[rule("==")]
    equals: Lexeme<Keyword>,

    #[rule("!=")]
    not_eq: Lexeme<Keyword>,

    #[rule(";")]
    semicolon: Lexeme<Keyword>,

    #[rule(",")]
    comma: Lexeme<Keyword>,

    #[rule("=>")]
    fat_arrow: Lexeme<Keyword>,

    #[rule("mod")]
    r#mod: Lexeme<Keyword>,

    #[rule("void")]
    void: Lexeme<Keyword>,

    #[rule("struct")]
    r#struct: Lexeme<Keyword>,

    #[rule("enum")]
    r#enum: Lexeme<Keyword>,

    #[rule("union")]
    union: Lexeme<Keyword>,

    #[rule("typedef")]
    typedef: Lexeme<Keyword>,

    #[rule("sizeof")]
    sizeof: Lexeme<Keyword>,

    #[rule("for")]
    r#for: Lexeme<Keyword>,

    #[rule("while")]
    r#while: Lexeme<Keyword>,

    #[rule("if")]
    r#if: Lexeme<Keyword>,

    #[rule("else")]
    r#else: Lexeme<Keyword>,

    #[rule("return")]
    r#return: Lexeme<Keyword>,

    #[rule("static")]
    r#static: Lexeme<Keyword>,

    #[rule("match")]
    r#match: Lexeme<Keyword>,

    #[rule("let")]
    r#let: Lexeme<Keyword>,

    #[rule("u8")]
    u8: Lexeme<Keyword>,

    #[rule("u16")]
    u16: Lexeme<Keyword>,

    #[rule("u32")]
    u32: Lexeme<Keyword>,

    #[rule("u64")]
    u64: Lexeme<Keyword>,

    #[rule("usize")]
    usize: Lexeme<Keyword>,

    #[rule("i8")]
    i8: Lexeme<Keyword>,

    #[rule("i16")]
    i16: Lexeme<Keyword>,

    #[rule("i32")]
    i32: Lexeme<Keyword>,

    #[rule("i64")]
    i64: Lexeme<Keyword>,

    #[rule("isize")]
    isize: Lexeme<Keyword>,

    #[rule("z8")]
    z8: Lexeme<Keyword>,

    #[rule("z16")]
    z16: Lexeme<Keyword>,

    #[rule("z32")]
    z32: Lexeme<Keyword>,

    #[rule("z64")]
    z64: Lexeme<Keyword>,

    #[rule("zsize")]
    zsize: Lexeme<Keyword>,

    #[rule("f32")]
    f32: Lexeme<Keyword>,

    #[rule("f64")]
    f64: Lexeme<Keyword>,

    #[rule("c32")]
    c32: Lexeme<Keyword>,

    #[rule("c64")]
    c64: Lexeme<Keyword>,

    #[rule("bool")]
    bool: Lexeme<Keyword>,

    #[rule("=")]
    set: Lexeme<Keyword>,

    #[rule("+=")]
    add_eq: Lexeme<Keyword>,

    #[rule("-=")]
    sub_eq: Lexeme<Keyword>,

    #[rule("*=")]
    mul_eq: Lexeme<Keyword>,

    #[rule("\\=")]
    div_eq: Lexeme<Keyword>,

    #[rule("%=")]
    mod_eq: Lexeme<Keyword>,

    #[rule(">>=")]
    shr_eq: Lexeme<Keyword>,

    #[rule("<<=")]
    shl_eq: Lexeme<Keyword>,

    #[rule("&=")]
    and_eq: Lexeme<Keyword>,

    #[rule("^=")]
    xor_eq: Lexeme<Keyword>,

    #[rule("|=")]
    or_eq: Lexeme<Keyword>,
}
