pub enum NodeType {
    Program(Vec<Box<Node>>),
    FunctionDeclaration { name: String, body: Box<Node> },
    VariableDeclaration { name: String, value: Box<Node>, data_type: DataType },
    ConstantDeclaration { name: String, value: Box<Node> },
    Expression(Box<Node>)
}

pub enum DataType {
    STRING,
    NUMBER,
}

pub struct Node {
    node_type: NodeType,
}
