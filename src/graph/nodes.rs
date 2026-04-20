use crate::tensor::Tensor;

#[derive(Debug)]
pub enum Node {
    Tensor(TensorNode),
    Operator(OperatorNode),
}

#[derive(Debug)]
pub struct TensorNode {
    tensor: Tensor,
}

#[derive(Debug)]
pub struct OperatorNode {
    // TODO: This will update to actually capture the data
    operator: String,
}

impl OperatorNode {
    pub fn new(operator: String) -> Self {
        OperatorNode { operator }
    }
}
