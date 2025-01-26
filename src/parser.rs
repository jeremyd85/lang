use std::sync::Arc;
use lasso;
use rug;
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
    
    fn new(interner: Arc<lasso::ThreadedRodeo>) -> Parser {
        Parser {
            nodes: NodeList::new(),
            interner
        }
    }
    
    fn parse(tokens: Vec<lexer::Token>) {
        
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
    Assignment(BinaryNode),
    IntLiteral(Box<rug::Integer>),
    Function(FunctionDef)
}

#[test]
fn ensure_same_size_node() {
    assert_eq!(size_of::<Box<rug::Integer>>(), size_of::<BinaryNode>());
    assert_eq!(size_of::<Node>(), 16);
}

