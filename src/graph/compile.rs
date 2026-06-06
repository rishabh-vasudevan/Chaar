use crate::dtype::Dtype;
use crate::graph::{
    graph_group::{GraphGroup, NodeIdx, TopoSorted},
    nodes::{BufferNode, GraphOperator, Node, OperatorNode},
};
use crate::shape_tracker::ShapeTracker;

#[derive(Debug, Clone)]
pub enum KernelOperator {
    Add,
    MatMul,
    NoOp,
}

impl KernelOperator {
    pub fn get_kernel_operator(graph_operator: GraphOperator) -> KernelOperator {
        match graph_operator {
            GraphOperator::Add => KernelOperator::Add,
            GraphOperator::MatMul => KernelOperator::MatMul,
            _ => KernelOperator::NoOp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Kernel {
    pub operator: KernelOperator,
    pub parents: Vec<NodeIdx>,
    pub output: NodeIdx,
}

#[derive(Debug)]
pub struct BufferToCreate {
    pub shape_tracker: ShapeTracker,
    pub dtype: Dtype,
    pub buffer_id: NodeIdx,
}

impl BufferToCreate {
    fn new(shape_tracker: ShapeTracker, dtype: Dtype, buffer_id: NodeIdx) -> Self {
        BufferToCreate {
            shape_tracker,
            dtype,
            buffer_id,
        }
    }
}

#[derive(Debug)]
pub struct ChaarIROne<'a> {
    pub graph_group: &'a GraphGroup,
    pub kernels: Vec<Kernel>,
    pub buffers_to_create: Vec<BufferToCreate>,
}

impl<'a> ChaarIROne<'a> {
    pub fn new(
        graph_group: &'a GraphGroup,
        kernels: Vec<Kernel>,
        buffers_to_create: Vec<BufferToCreate>,
    ) -> Self {
        ChaarIROne {
            graph_group,
            kernels,
            buffers_to_create,
        }
    }
}

#[derive(Debug)]
pub struct ChaarIRTwo {}

#[derive(Debug)]
pub enum ChaarIRS<'a> {
    ChaarIROne(ChaarIROne<'a>),
    ChaarIRTwo(ChaarIRTwo),
}

impl<'a> ChaarIRS<'a> {
    pub fn search_compile_and_execute(graph_group: &'a GraphGroup) {
        todo!()
    }

    pub fn compile(graph_group: &'a GraphGroup, graph_index: usize) -> ChaarIRS {
        let chaar_ir_one = Self::level_one(graph_group, graph_index);
        ChaarIRS::ChaarIROne(chaar_ir_one)
    }

    fn level_one(graph_group: &'a GraphGroup, graph_index: usize) -> ChaarIROne {
        let sorted_graph = graph_group.topo_sort(graph_index);

        let operator_nodes = sorted_graph
            .iter()
            .filter_map(|node| match graph_group.get_node(node.node_id) {
                Node::Operator(operator_node) => Some((node, operator_node)),
                _ => None,
            })
            .collect::<Vec<(&TopoSorted, &OperatorNode)>>();

        let buffer_nodes = sorted_graph
            .iter()
            .filter_map(|node| match graph_group.get_node(node.node_id) {
                Node::InputBuffer(buffer_node) => Some((node, buffer_node)),
                Node::IntermittentBuffer(buffer_node) => Some((node, buffer_node)),
                _ => None,
            })
            .collect::<Vec<(&TopoSorted, &BufferNode)>>();

        let mut kernels = Vec::new();
        for (topo_sorted_node, operator_node) in operator_nodes {
            let kernel_operator = KernelOperator::get_kernel_operator(operator_node.op);

            kernels.push(Kernel {
                operator: kernel_operator,
                parents: topo_sorted_node.parents.clone(),
                output: operator_node.output_buffer,
            });
        }

        let buffers_to_create = buffer_nodes
            .iter()
            .map(|(node, buffer_node)| {
                (BufferToCreate::new(
                    buffer_node.shape.clone(),
                    buffer_node.dtype.clone(),
                    node.node_id,
                ))
            })
            .collect();

        ChaarIROne::new(graph_group, kernels, buffers_to_create)
    }

    fn level_two(_chaar_ir_one: ChaarIROne) -> ChaarIRS {
        ChaarIRS::ChaarIRTwo(ChaarIRTwo {})
    }
}
