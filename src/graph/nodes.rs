use crate::{graph::graph_group::NodeIdx, shape_tracker::ShapeTracker, tensor::Tensor};

#[derive(Debug)]
pub enum Node {
    Tensor(TensorNode),
    Operator(OperatorNode),
    InputBuffer(BufferNode),
    TransitBuffer(BufferNode),
    Output,
}

impl Node {
    pub fn get_label(&self) -> String {
        match self {
            Self::Tensor(node) => node.label.clone(),
            Self::Operator(node) => node.label.clone(),
            Self::InputBuffer(node) => node.label.clone(),
            Self::TransitBuffer(node) => node.label.to_string(),
            Self::Output => "Output".to_string(),
        }
    }

    // TODO: consider custor errors instead of strings to improve the error hadling
    pub fn get_operator(&self) -> Result<GraphOperator, String> {
        match self {
            Node::Operator(operator_node) => Ok(operator_node.op),
            _ => Err("Node is not an operator node".to_string()),
        }
    }

    pub fn get_shape(&self) -> Result<ShapeTracker, String> {
        match self {
            Node::Tensor(tensor_node) => Ok(tensor_node.tensor.shape.clone()),
            Node::InputBuffer(buffer_node) => Ok(buffer_node.shape.clone()),
            Node::TransitBuffer(buffer_node) => Ok(buffer_node.shape.clone()),
            _ => Err("Node does not have a shape value".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct BufferNode {
    label: String,
    shape: ShapeTracker,
}

impl BufferNode {
    pub fn new(label: String, shape: ShapeTracker) -> Self {
        BufferNode { label, shape }
    }
}

#[derive(Debug)]
pub struct TensorNode {
    pub tensor: Tensor,
    pub label: String,
}

impl TensorNode {
    pub fn new(tensor: Tensor, label: String) -> Self {
        TensorNode { tensor, label }
    }
}

// TODO: is this clone needed?
#[derive(Debug, Clone, Copy)]
pub enum GraphOperator {
    Add,
    MatMul,
    ElementWiseMul,
    Dummy,
}

impl GraphOperator {
    pub fn value(&self) -> usize {
        match self {
            GraphOperator::Add => 2,
            GraphOperator::MatMul => 2,
            GraphOperator::ElementWiseMul => 2,
            GraphOperator::Dummy => 0,
        }
    }
    pub fn label(&self) -> String {
        match self {
            GraphOperator::Add => "Add".to_string(),
            GraphOperator::MatMul => "MatMul".to_string(),
            GraphOperator::ElementWiseMul => "ElementWiseMul".to_string(),
            GraphOperator::Dummy => "Dummy".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct OperatorNode {
    pub op: GraphOperator,
    pub label: String,
    pub output_buffer: NodeIdx,
}

impl OperatorNode {
    pub fn new(op: GraphOperator, output_buffer: NodeIdx, label: String) -> Self {
        OperatorNode {
            op,
            label,
            output_buffer,
        }
    }
}
