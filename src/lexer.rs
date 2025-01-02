use std::iter::Peekable;
use logos::{Lexer, Logos};

#[derive(Clone, Debug, PartialEq, Logos)]
#[logos()]
pub enum Tokens<'source> {
    #[regex(r"[-+]?((0b[01]{1,64})|0x[0-9ABCDEFabcdef]{1,32}|0o[0-7]{1,48}|[0-9]+)"), |i| i.slice().parse::<i64>().unwrap()]
    Integer(i64),
    #[regex(r"[-+]?(infinity|nan|inf|([0-9]*(\.([0-9]+)?)?([e|E]?[+-][0-9]+)?))"), |f| f.slice().parse::<f64>().unwrap()]
    Float(f64),
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Boolean(bool),
    #[token("nil")]
    Nil,

    #[regex("_*([a-zA-Z][a-zA-Z0-9]+)")]
    Identifier,

    // Keywords
    #[token("do")]
    Do,
    #[token("end")]
    End,
    #[token("catch")]
    Catch,
    #[token("rescue")]
    Rescue,
    #[token("raise")]
    Raise,
    #[token("after")]
    After,
    #[token("else")]
    Else,
    #[token("construct")]
    Construct,
    #[token("figure")]
    Figure,
    #[token("protocol")]
    Protocol,
    #[token("union")]
    Union,
    #[token("mod")]
    Module,
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("when")]
    When,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("in")]
    In,
    #[token("import")]
    Import,
    #[token("requires")]
    Requires,
    #[token("defaults")]
    Defaults,
    #[token("return")]
    Return,
    #[token("as")]
    As,

    #[token(".")]
    KeyDefinition,
    #[token(":")]
    Colon, // :
    #[token("><")]
    Concatenate,
    #[token("|<<")]
    ComptimeCompose,
    #[token(">>")]
    Special,
    #[token("+")]
    Plus,
    #[token("~")]
    Tilde, // ~
    #[token("@")]
    At, // @
    #[token("$")]
    DollarSign,
    #[token(",")]
    Comma,
    #[token("'")]
    Label,
    #[token("::")]
    ThingSeperator,
    #[token("*")]
    Star,

    #[token("(")]
    ParenthesesOpen,
    #[token(")")]
    ParenthesesClose,
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token("${")]
    FormatOpen,

    #[token(r#"""#)]
    Quote,

    #[token(" ")]
    Space,
    #[token("\t")]
    Tab,
    #[token("\n")]
    Newline,
    #[regex(r"#(.*)?[^\n\r]", |lex| lex.slice())]
    Comment(&'source str),
}

pub enum FallThroughToken<'source> {
    Token(Tokens<'source>),
    Other(&'source str),
}

pub struct BoreumLexer<'source> {
    lexer: Peekable<Lexer<'source, Tokens<'source>>>
}

impl<'source> BoreumLexer<'source> {
    pub fn new(src: &'source str) -> Self {
        Self {
            lexer: Pee,
        }
    }
}

impl<'source> Iterator for BoreumLexer<'source> {
    type Item = FallThroughToken<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}
