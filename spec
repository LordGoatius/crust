tokens:
    keyword,
    ident,
    constant,
    literal,
    operator,
    punctuation,

keyword:
    "mod", # No C Headers, Rust mod system will be preferred
    "struct",
    "enum",
    "union",   # reserved
    "typedef",
    "sizeof",
    "for",
    "while",
    "if",
    "else",
    "return",
    "static",
    "match",
    "let",
    "u8", # Unsigned Int
    "u16",
    "u32",
    "u64",
    "usize",
    "i8", # Signed Int
    "i16", 
    "i32",
    "i64",
    "isize",
    "z8",  # Gaussian Int
    "z16",
    "z32",
    "z64",
    "zsize",
    "f32", # Floating Point
    "f64",
    "c32", # Complex Numbers
    "c64",
    "=",   # Assignment Ops
    "+=",
    "-=",
    "*=",
    "\=",
    "%="
    "<<=",
    ">>=",
    "&=", 
    "^=",
    "|="

nondigit: "_",
"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", 
"n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", 
"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", 
"N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"

digit: "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"

ident:
    (nondigit+) + (nondigit | digit)*

constant:
    int_const,
    floating_const,
    enum_const,
    char_const,
    bool_const,
    ptr_const,

    int_const: digit+
    floating_const: (int_const) + "." +int_const
    enum_const: ident
    char_const: "'" + (nondigit | digit) + "'" # no escapes or anything yet
                                               # yes that means no " in chars, but
                                               # we'll see how `ilex` can make
                                               # strings easy maybe
    bool_const: "true" | "false",
    ptr_const: "null"

literal: """ + char_const* +  """

operators:
    "*",
    ".",
    "->",
    "++",
    "--",
    "+",
    "-",
    "/",
    "%",
    "&",
    "|",
    "^",
    "~",
    "!",
    "&&",
    "||",
    "?",
    ":",
    ">>",
    "<<",
    "<",
    ">",
    "<=",
    ">=",
    "==",
    "!=",

puncuation:
    ";",
    "{",
    "}",
    "[",
    "]",
    "(",
    ")",
    ",",
    "=>",


