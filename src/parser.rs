use std::str::FromStr;
use std::sync::Arc;
use lasso;
use thiserror::Error;
use crate::lexer;
use crate::lexer::{TokenKind, TokenStream};


#[derive(Debug, Error)]
struct CompileError {
    start_pos: u32,
    end_pos: u32,
    message: String
}


struct NodeList {
    nodes: Vec<Node>
}

impl NodeList {
    fn new() -> NodeList {
        NodeList {
            nodes: Vec::new()
        }
    }

    fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn get(&self, index: NodeIndex) -> &Node {
        &self.nodes[index as usize]
    }
}

type NodeIndex = u32;

struct Parser<'a> {
    nodes: NodeList,
    interner: Arc<lasso::ThreadedRodeo>,
    stream: &'a mut TokenStream<'a>
}

impl<'a> Parser<'a> {
    
    pub fn new(token_stream: &'a mut TokenStream<'a>, interner: Arc<lasso::ThreadedRodeo>) -> Parser<'a> {
        Parser {
            nodes: NodeList::new(),
            stream: token_stream,
            interner
        }
    }
    
    pub fn parse_expression(&mut self) -> Option<NodeIndex> {
        let lhs = self.parse_term();
        if lhs.is_none() {
            return None;
        }
        self.stream.consume(TokenKind::Plus);
        None
    }
    
    pub fn parse_term(&mut self) -> Option<NodeIndex> {
        None
    }
    
    pub fn parse_int_literal(&mut self) -> Option<NodeIndex> {
        
        let s = self.interner.resolve(&token.value);
        if let Ok(i64::from_str(s)) {
            
        }
    }
    
    pub fn execute(&mut self, tokens: Vec<lexer::Token>) {
        for token in tokens {
            match token.kind {
                lexer::TokenKind::Number => {
                    


                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BinaryNode {
    lhs: NodeIndex,
    rhs: NodeIndex,
}

#[derive(Clone, Copy)]
struct FunctionDef {
    signature: NodeIndex, // Signature
    body: NodeIndex, // FunctionBody
}

#[derive(Clone, Copy)]
struct Signature {
    name: lasso::Spur, // Identifier
    parameters: NodeIndex, // Parameters
}

#[derive(Clone, Copy)]
struct Parameters {
    parameter: NodeIndex, // Parameter
    next: NodeIndex // Parameters
    
}

#[derive(Clone, Copy)]
struct Parameter {
    name: lasso::Spur, // Identifier
    ty: lasso::Spur, // Identifier
}

#[derive(Clone)]
enum Node {
    Add(BinaryNode),
    Subtract(BinaryNode),
    Multiply(BinaryNode),
    Divide(BinaryNode),
    NotEqual(BinaryNode),
    Equal(BinaryNode),
    LessThan(BinaryNode),
    LessThanOrEqual(BinaryNode),
    GreaterThan(BinaryNode),
    GreaterThanOrEqual(BinaryNode),
    LogicalAnd(BinaryNode),
    LogicalOr(BinaryNode),
    Assignment(BinaryNode),
    IntLiteral(i64),
    FloatLiteral(f64),
    Function(FunctionDef)
}

#[test]
fn ensure_same_size_node() {
    assert_eq!(size_of::<Node>(), 16);
}

#[test]
fn addition() {
    let mut interner = Arc::new(lasso::ThreadedRodeo::new());
    let mut p = Parser::new(interner.clone());
    let stream = lexer::TokenStream::new(lexer::Lexer::new("1 + 2", &mut interner).lex().as_slice());
    p.execute(lexer::Lexer::new("1", &mut interner).lex());
    if let Node::IntLiteral(v) = p.nodes.get(0) {
        assert_eq!(v.to_string(), "1");
    }
}
