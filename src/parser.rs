use std::sync::Arc;
use lasso;
use rug::{Integer, Assign};
use crate::lexer;

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

struct Parser {
    nodes: NodeList,
    interner: Arc<lasso::ThreadedRodeo>,
}

impl Parser {
    
    pub fn new(interner: Arc<lasso::ThreadedRodeo>) -> Parser {
        Parser {
            nodes: NodeList::new(),
            interner
        }
    }
    
    pub fn execute(&mut self, tokens: Vec<lexer::Token>) {
        for token in tokens {
            match token.kind {
                lexer::TokenKind::Number => {
                    let mut i = Integer::new();
                    i.assign(Integer::parse(self.interner.resolve(&token.value)).unwrap());
                    self.nodes.add(Node::IntLiteral(Box::new(i)));


                }
                _ => {}
            }
        }
    }
}

struct BinaryNode {
    lhs: NodeIndex,
    rhs: NodeIndex,
}

struct FunctionDef {
    signature: NodeIndex, // Signature
    body: NodeIndex, // FunctionBody
}

struct Signature {
    name: lasso::Spur, // Identifier
    parameters: NodeIndex, // Parameters
}

struct Parameters {
    parameter: NodeIndex, // Parameter
    next: NodeIndex // Parameters
    
}

struct Parameter {
    name: lasso::Spur, // Identifier
    ty: lasso::Spur, // Identifier
}

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
    IntLiteral(Box<rug::Integer>),
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
