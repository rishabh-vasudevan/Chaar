use crate::graph::{
    graph_group::{GraphGroup, NodeIdx, TopoSorted},
    nodes::{GraphOperator, Node, OperatorNode},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Kernel {
    operator: KernelOperator,
    parents: Vec<NodeIdx>,
}

#[derive(Debug, Default)]
pub struct ChaarIROne {
    kernels: Vec<Kernel>,
}

#[derive(Debug)]
pub struct ChaarIRTwo {}

#[derive(Debug)]
pub enum ChaarIRS {
    ChaarIROne(ChaarIROne),
    ChaarIRTwo(ChaarIRTwo),
}

impl ChaarIRS {
    pub fn search_and_compile(graph_group: &GraphGroup) {
        todo!()
    }
    pub fn compile(graph_group: &GraphGroup, graph_index: usize) -> ChaarIRS {
        let chaar_ir_one = Self::level_one(graph_group, graph_index);
        ChaarIRS::ChaarIROne(chaar_ir_one)
    }

    fn level_one(graph_group: &GraphGroup, graph_index: usize) -> ChaarIROne {
        let sorted_graph = graph_group.topo_sort(graph_index);

        // NOTE: This will only have operator nodes, is there a better way to do this?
        let operator_nodes = sorted_graph
            .iter()
            .filter_map(|node| match graph_group.get_node(node.node_id) {
                Node::Operator(operator_node) => Some((node, operator_node)),
                _ => None,
            })
            .collect::<Vec<(&TopoSorted, &OperatorNode)>>();

        let mut chaar_ir_one = ChaarIROne::default();

        for (topo_sorted_node, operator_node) in operator_nodes {
            let kernel_operator = KernelOperator::get_kernel_operator(operator_node.op);

            chaar_ir_one.kernels.push(Kernel {
                operator: kernel_operator,
                parents: topo_sorted_node.parents.clone(),
                // TODO: This needs a child as well
            });
        }
        chaar_ir_one
    }

    fn level_two(chaar_ir_one: ChaarIROne) -> ChaarIRS {
        println!("Reached Level Two");
        ChaarIRS::ChaarIRTwo(ChaarIRTwo {})
    }
}
