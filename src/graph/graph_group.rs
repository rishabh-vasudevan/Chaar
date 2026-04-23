use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use crate::graph::{
    compile::ChaarIRS,
    nodes::{BufferNode, Node, Operator, OperatorNode, TensorNode},
};
use crate::tensor::Tensor;

type NodeIdx = usize;
type Edge = (NodeIdx, NodeIdx); // (Source, Destination)
const DEFAULT_GRAPH: usize = 0;

#[derive(Debug, Clone)]
pub struct Graph {
    edges: Vec<Edge>,
    children: Vec<Graph>,
}

impl Graph {
    fn new(edges: Vec<(usize, usize)>) -> Self {
        Graph {
            edges,
            children: Vec::new(),
        }
    }
}

// NOTE: We are doing things in an arena, that is why it is okay not have refrences of edge nodes
// The problem in the general way is that if a node is dropped and it still had an edge that can
// run into undefined behaviour.And if we want to start dropping nodes, in that case we will
// have to keep references, so that we don't drop nodes that are being used in other graphs. For
// now all deletions should happen at the same time, or rather just let the program complete and
// never delete the nodes.
#[derive(Debug, Default)]
pub struct GraphGroup {
    nodes: Vec<Node>,
    graphs: Vec<Graph>,
}

pub struct TopoSorted {
    pub layer: usize,
    pub node_id: usize,
    pub parents: Vec<usize>,
}

impl GraphGroup {
    pub fn get_nodes(&self) -> &Vec<Node> {
        self.nodes.as_ref()
    }

    pub fn get_edges(&self, graph_index: usize) -> &Vec<(usize, usize)> {
        self.graphs[graph_index].edges.as_ref()
    }

    pub fn num_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_graph_latest_index(&self) -> usize {
        self.graphs.len() - 1
    }

    pub fn get_node_latest_index(&self) -> usize {
        self.nodes.len() - 1
    }

    pub fn add_graph(&mut self, edges: Vec<(usize, usize)>) -> usize {
        if !edges.is_empty() {
            assert!(
                edges
                    .iter()
                    .all(|x| x.0 < self.num_of_nodes() && x.1 < self.num_of_nodes()),
                "Graph inputs mismatch"
            );
        }
        // TODO: Check that graph does not have cycle
        self.graphs.push(Graph::new(edges));
        self.get_graph_latest_index()
    }

    pub fn add_tensor(&mut self, tensor: Tensor, label: String) -> usize {
        self.nodes
            .push(Node::Tensor(TensorNode::new(tensor, label)));
        self.get_node_latest_index()
    }

    pub fn add_operator(
        &mut self,
        graph_index: usize,
        operator: Operator,
        mut operands: Vec<usize>,
        buffer_label: String,
    ) -> usize {
        assert!(operands.len() == operator.value());
        //TODO: remove this check for same elements later to have a + a also as valid
        operands.sort();
        operands.dedup();

        self.nodes.push(Node::Operator(OperatorNode::new(
            operator,
            operator.label(),
        )));
        let operator_index = self.nodes.len() - 1;
        for operand in operands {
            self.graphs[graph_index]
                .edges
                .push((operand, operator_index));
        }
        self.nodes.push(Node::Buffer(BufferNode::new(buffer_label)));
        let buffer_index = self.get_node_latest_index();
        self.graphs[graph_index]
            .edges
            .push((operator_index, buffer_index));

        buffer_index
    }

    pub fn topo_sort(&self, graph_index: usize) -> Vec<TopoSorted> {
        let graph = &self.graphs[graph_index];
        let mut incoming_edges_count: HashMap<usize, usize> = HashMap::new();
        let mut outgoing_edges: HashMap<usize, Vec<usize>> = HashMap::new();

        for (src, dest) in graph.edges.iter() {
            if src == dest {
                continue;
            }
            incoming_edges_count.entry(*src).or_insert(0);
            *incoming_edges_count.entry(*dest).or_insert(0) += 1;
            outgoing_edges.entry(*src).or_default().push(*dest);
        }

        let mut no_incoming_edges: VecDeque<usize> = VecDeque::default();

        for (src, src_incoming) in incoming_edges_count.iter() {
            if *src_incoming == 0 {
                no_incoming_edges.push_back(*src);
            }
        }

        let mut sorted_nodes: Vec<usize> = Vec::new();

        let mut layers: Vec<usize> = vec![0; self.num_of_nodes()];
        let mut parents: Vec<Vec<usize>> = vec![vec![]; self.num_of_nodes()];

        while let Some(src) = no_incoming_edges.pop_back() {
            sorted_nodes.push(src);

            incoming_edges_count.remove(&src);

            for neighbour in outgoing_edges.get(&src).unwrap_or(&vec![]) {
                if let Some(count) = incoming_edges_count.get_mut(neighbour) {
                    parents[*neighbour].push(src);
                    *count -= 1;
                    if *count == 0 {
                        layers[*neighbour] = layers[*neighbour].max(layers[src] + 1);
                        incoming_edges_count.remove(neighbour);
                        no_incoming_edges.push_back(*neighbour);
                    }
                }
            }
        }

        if !incoming_edges_count.is_empty() {
            panic!("Graph is not a DAG");
        }

        sorted_nodes
            .iter()
            .map(|node_id| TopoSorted {
                layer: layers[*node_id],
                node_id: *node_id,
                parents: parents[*node_id].clone(),
            })
            .collect()
    }

    pub fn compile(&mut self, graph_index: usize) {
        ChaarIRS::compile(self, DEFAULT_GRAPH);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::*;
    use crate::graph::nodes::*;
    use crate::shape_tracker::*;
    use crate::tensor::*;

    #[test]
    fn add_two_tensors() {
        let mut graph_group = GraphGroup::default();

        //                C
        //               ADD
        //  A [1, 2, 3, 4]  B [5, 6, 7, 8]

        let graph = graph_group.add_graph(vec![]);

        let a = Tensor::new(
            Dtype::Float32(vec![1.0, 2.0, 3.0, 4.0]),
            ShapeTracker::new(vec![4], vec![1]),
        );

        let b = Tensor::new(
            Dtype::Float32(vec![5.0, 6.0, 7.0, 8.0]),
            ShapeTracker::new(vec![4], vec![1]),
        );

        let a = graph_group.add_tensor(a, stringify!(a).to_string());
        let b = graph_group.add_tensor(b, stringify!(b).to_string());

        let c =
            graph_group.add_operator(graph, Operator::Add, vec![a, b], stringify!(c).to_string());

        let d =
            graph_group.add_operator(graph, Operator::Add, vec![c, a], stringify!(d).to_string());

        println!("Nodes: {:?}", graph_group);
        println!("Edges: {:?}", graph_group.graphs[graph].edges);
    }
}
