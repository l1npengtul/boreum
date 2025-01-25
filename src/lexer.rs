use std::iter::Peekable;
use cstree::RawSyntaxKind;
use logos::{Lexer, Logos};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::syntax::BoreumLang;

pub enum LexerError {

}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Logos, ToPrimitive, FromPrimitive)]
pub enum Tokens {
    #[regex(r"[-+]?((0b[01][01_]+)|0x[0-9ABCDEFabcdef][0-9ABCDEFabcdef_]+|0o[0-7][0-7_]+|[0-9][0-9_]+)")]
    Integer,
    #[regex(r"[-+]?(infinity|nan|inf|([0-9_]*(\.([0-9_]+)?)?([e|E]?[+-][0-9_]+)?))")]
    Float,
    #[token("false")]
    #[token("true")]
    Boolean,
    #[token("nil")]
    Nil,

    #[regex("_*([a-zA-Z][a-zA-Z0-9]+)")]
    Identifier,

    // Keywords
    #[token("do")]
    Do,
    #[token("end")]
    End,
    #[token("soothe")]
    Soothe,
    #[token("rescue")]
    Rescue,
    #[token("panic")]
    Panic,
    #[token("after")]
    After,
    #[token("else")]
    Else,
    #[token("construct")]
    Construct,
    #[token("enum")]
    Enum,
    #[token("figure")]
    Figure,
    #[token("union")]
    Union,
    #[token("trait")]
    Trait,
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
    #[token("is")]
    Is,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("external")]
    External,
    #[token("requires")]
    Requires,
    #[token("defaults")]
    Defaults,
    #[token("return")]
    Return,
    #[token("as")]
    As,
    #[token("impl")]
    Impl,
    #[token("nursery")]
    Nursery,
    #[token("mesmerizer")]
    Mesmerizer,
    #[token("mesmerize")]
    Mesmerize,
    #[token("macro")]
    Macro,
    #[token("pub")]
    Public,
    #[token("comptime")]
    Comptime,
    #[token("const")]
    Const,
    #[token("pure")]
    Pure,
    #[token("special")]
    Special,
    #[token("field")]
    Field,
    #[token("constraints")]
    Constraints,
    #[token("loop")]
    Loop,
    #[token("recurse")]
    Recurse,
    #[token("unsafe")]
    Unsafe,
    #[token("local")]
    Local,
    #[token("isolated")]
    Isolated,
    #[token("immut")]
    Immut,

    #[token("><")]
    SakanaTail,
    #[token("><>")]
    ForwardSakana,
    #[token("<><")]
    BackwardSakana,
    #[token(">>")]
    SubBlock,
    #[token("=>")]
    Arrow,

    #[token(".")]
    Period,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("/")]
    ForwardSlash,
    #[token("$")]
    GlobalValue,
    #[token("=")]
    EqualSign,
    #[token("..")]
    DoublePeriod,
    #[token("...")]
    TriplePeriod,
    #[token("!")]
    ExclamationMark,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("|")]
    Pipe,

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
    #[token("<")]
    AngleOpen,
    #[token(">")]
    AngleClose,

    #[token("@[")]
    AnnotationOpen,
    #[token("${")]
    FormatOpen,
    #[token("~")]
    Destructuring,
    #[token("*")]
    Clone,

    #[token(r#"""#)]
    Quote,
    #[token("''")]
    MultiLineQuote, // i love stealing from Nix
    #[token("'")]
    Label,
    #[regex(r#"r"([^\\"]*)""#)]
    #[regex(r##"r#"([^\\"]*)"#"##)]
    #[regex(r###"r##"([^\\"]*)"##"###)]
    #[regex(r####"r###"([^\\"]*)"###"####)]
    #[regex(r#####"r####"([^\\"]*)"####"#####)]
    #[regex(r######"r#####"([^\\"]*)"#####"######)]
    #[regex(r#######"r######"([^\\"]*)"######"#######)]
    RawString,

    #[regex(" +")]
    Whitespace,
    #[token("\n")]
    Newline,
    #[regex(r"#(.*)?[^\n\r]")]
    Comment,
    #[regex(r"##(.*)?[^\n\r]")]
    Documentation,

    // parser constructs
    Root,
}

impl cstree::Syntax for Tokens {
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from_u32(raw.0).unwrap()
    }

    fn into_raw(self) -> RawSyntaxKind {
        self.to_u32().unwrap().into()
    }

    fn static_text(self) -> Option<&'static str> {
        match self {
            Tokens::Do => "do",
            Tokens::End => "end",
            Tokens::Soothe => "soothe",
            Tokens::Rescue => "rescue",
            Tokens::Panic => "panic",
            Tokens::After => "after",
            Tokens::Else => "else",
            Tokens::Construct => "construct",
            Tokens::Enum => "enum",
            Tokens::Figure => "figure",
            Tokens::Union => "union",
            Tokens::Trait => "trait",
            Tokens::Module => "module",
            Tokens::Let => "let",
            Tokens::Fn => "fn",
            Tokens::When => "when",
            Tokens::And => "and",
            Tokens::Or => "or",
            Tokens::Not => "not",
            Tokens::In => "in",
            Tokens::Is => "is",
            Tokens::Import => "import",
            Tokens::Export => "export",
            Tokens::External => "external",
            Tokens::Requires => "requires",
            Tokens::Defaults => "defaults",
            Tokens::Return => "return",
            Tokens::As => "as",
            Tokens::Impl => "impl",
            Tokens::Nursery => "nursery",
            Tokens::Mesmerizer => "mesmerizer",
            Tokens::Mesmerize => "mesmerize",
            Tokens::Macro => "macro",
            Tokens::Public => "public",
            Tokens::Comptime => "comptime",
            Tokens::Const => "const",
            Tokens::Pure => "pure",
            Tokens::Special => "special",
            Tokens::Field => "field",
            Tokens::Constraints => "constraints",
            Tokens::Loop => "loop",
            Tokens::Recurse => "recurse",
            Tokens::Unsafe => "unsafe",
            Tokens::Local => "local",
            Tokens::Isolated => "isolated",
            Tokens::Immut => "immut",
            Tokens::SakanaTail => "><",
            Tokens::ForwardSakana => "><>",
            Tokens::BackwardSakana => "<><",
            Tokens::SubBlock => ">>",
            Tokens::Arrow => "=>",
            Tokens::Period => ".",
            Tokens::Colon => ":",
            Tokens::Comma => ",",
            Tokens::ForwardSlash => "/",
            Tokens::GlobalValue => "@",
            Tokens::EqualSign => "=",
            Tokens::DoublePeriod => "..",
            Tokens::TriplePeriod => "...",
            Tokens::ExclamationMark => "!",
            Tokens::Plus => "+",
            Tokens::Minus => "-",
            Tokens::Pipe => "|",
            Tokens::ParenthesesOpen => "(",
            Tokens::ParenthesesClose => ")",
            Tokens::SquareOpen => "[",
            Tokens::SquareClose => "]",
            Tokens::CurlyOpen => "{",
            Tokens::CurlyClose => "}",
            Tokens::AngleOpen => "<",
            Tokens::AngleClose => ">",
            Tokens::AnnotationOpen => "@[",
            Tokens::FormatOpen => "${",
            Tokens::Destructuring => "~",
            Tokens::Clone => "*",
            Tokens::Quote => "\"",
            Tokens::MultiLineQuote => "''",
            Tokens::Label => "'",
            Tokens::Newline => "\n",
            _ => ""
        }.map(|x| {
            if x == "" {
                None
            }
            else {
                Some(x)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use logos::Logos;
    use crate::lexer::Tokens;

    fn sesbian_lex(input: &str, token: Tokens) {
        let mut lexer = Tokens::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(token)));
        assert_eq!(lexer.slice(), input);
    }

    macro_rules! lex_test {
        ( $( $case:ident { $($text:literal : $token:expr,)* } )+ ) => {
            paste::paste! {
                $(
                fn [< lex_ $case >]() {
                    $(
                    sesbian_lex($text, $token)
                    )*
                }
                )+
            }
        }
    }

    lex_test!(
        integer {
            "-123": Tokens::Integer,
            "12_3": Tokens::Integer,
            "+1__23": Tokens::Integer,
            "0xDEADBEEF": Tokens::Integer,
            "0x777": Tokens::Integer,
        }
        float {
            "infinity": Tokens::Float,
            "inf": Tokens::Float,
            "nan": Tokens::Float,
            "-12314.363_24252e123_25": Tokens::Float,
            "+1.0": Tokens::Float,
        }
        bool {
            "true": Tokens::Boolean,
            "false": Tokens::Boolean,
        }
        nil {
            "nil": Tokens::Nil,
        }
        identifier {
            "_aA12452": Tokens::Identifier,
            "turtle": Tokens::Identifier,
            "FUK": Tokens::Identifier,
        }
        keywords {
            "do": Tokens::Do,
            "end": Tokens::End,
            "soothe": Tokens::Soothe,
            "rescue": Tokens::Rescue,
            "panic": Tokens::Panic,
            "after": Tokens::After,
            "else": Tokens::Else,
            "construct": Tokens::Construct,
            "enum": Tokens::Enum,
            "figure": Tokens::Figure,
            "union": Tokens::Union,
            "trait": Tokens::Trait,
            "mod": Tokens::Module,
            "let": Tokens::Let,
            "fn": Tokens::Fn,
            "when": Tokens::When,
            "and": Tokens::And,
            "or": Tokens::Or,
            "not": Tokens::Not,
            "in": Tokens::In,
            "is": Tokens::Is,
            "import": Tokens::Import,
            "export": Tokens::Export,
            "external": Tokens::External,
            "requires": Tokens::Requires,
            "defaults": Tokens::Defaults,
            "return": Tokens::Return,
            "as": Tokens::As,
            "impl": Tokens::Impl,
            "nursery": Tokens::Nursery,
            "mesmerizer": Tokens::Mesmerizer,
            "mesmerize": Tokens::Mesmerize,
            "macro": Tokens::Macro,
            "public": Tokens::Public,
            "comptime": Tokens::Comptime,
            "const": Tokens::Const,
            "pure": Tokens::Pure,
            "special": Tokens::Special,
            "field": Tokens::Field,
            "constraints": Tokens::Constraints,
            "loop": Tokens::Loop,
            "recurse": Tokens::Recurse,
            "unsafe": Tokens::Unsafe,
            "local": Tokens::Local,
            "isolated": Tokens::Isolated,
            "immut": Tokens::Immut,
        }
        operators {
            "><": Tokens::SakanaTail,
            "><>": Tokens::ForwardSakana,
            "<><": Tokens::BackwardSakana,
            ">>": Tokens::SubBlock,
            "=>": Tokens::Arrow,
        }
    );
}
