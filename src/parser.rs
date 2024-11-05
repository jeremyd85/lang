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
}

struct BinaryNode {
    lhs: NodeIndex,
    rhs: NodeIndex,
}

enum Node {
    Add(BinaryNode),
    Subtract(BinaryNode),
    Multiply(BinaryNode),
    Divide(BinaryNode),
    Assignment(BinaryNode),
    IntLiteral(i64),
    FunctionDef()
}