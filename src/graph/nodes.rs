use crate::tensor::Tensor;

#[derive(Debug)]
pub enum Node {
    Tensor(TensorNode),
    Operator(OperatorNode),
    Buffer,
}

#[derive(Debug)]
pub struct TensorNode {
    tensor: Tensor,
}

impl TensorNode {
    pub fn new(tensor: Tensor) -> Self {
        TensorNode { tensor }
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    MatMul,
    ElementWiseMul,
    Dummy,
}

impl Operator {
    fn value(&self) -> usize {
        match self {
            Operator::Add => 2,
            Operator::MatMul => 2,
            Operator::ElementWiseMul => 2,
            Operator::Dummy => 0,
        }
    }
}

#[derive(Debug)]
pub struct OperatorNode {
    op: Operator,
}

impl OperatorNode {
    pub fn new(op: Operator) -> Self {
        OperatorNode { op }
    }
}
