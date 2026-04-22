use crate::tensor::Tensor;

#[derive(Debug)]
pub enum Node {
    Tensor(TensorNode),
    Operator(OperatorNode),
    Buffer(BufferNode),
    Output,
}

impl Node {
    pub fn get_label(&self) -> String {
        match self {
            Self::Tensor(node) => node.label.clone(),
            Self::Operator(node) => node.label.clone(),
            Self::Buffer(node) => node.label.clone(),
            Self::Output => "Output".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct BufferNode {
    label: String,
}

impl BufferNode {
    pub fn new(label: String) -> Self {
        BufferNode { label }
    }
}

#[derive(Debug)]
pub struct TensorNode {
    tensor: Tensor,
    label: String,
}

impl TensorNode {
    pub fn new(tensor: Tensor, label: String) -> Self {
        TensorNode { tensor, label }
    }
}

// TODO: is this clone needed?
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    MatMul,
    ElementWiseMul,
    Dummy,
}

impl Operator {
    pub fn value(&self) -> usize {
        match self {
            Operator::Add => 2,
            Operator::MatMul => 2,
            Operator::ElementWiseMul => 2,
            Operator::Dummy => 0,
        }
    }
    pub fn label(&self) -> String {
        match self {
            Operator::Add => "Add".to_string(),
            Operator::MatMul => "MatMul".to_string(),
            Operator::ElementWiseMul => "ElementWiseMul".to_string(),
            Operator::Dummy => "Dummy".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct OperatorNode {
    op: Operator,
    label: String,
}

impl OperatorNode {
    pub fn new(op: Operator, label: String) -> Self {
        OperatorNode { op, label }
    }
}
