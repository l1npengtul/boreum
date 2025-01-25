use cstree::build::GreenNodeBuilder;
use cstree::green::GreenNode;
use logos::{Lexer, Logos};
use crate::lexer::Tokens;

pub struct Parser<'source> {
    lexer: Lexer<'source, Tokens>,
    builder: GreenNodeBuilder<'static, 'static, Tokens>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Tokens::lexer(source),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> ParsedTree {
    }

    pub fn start()
}

pub struct ParsedTree {
    green_node: GreenNode
}