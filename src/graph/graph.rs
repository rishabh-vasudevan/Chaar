use crate::graph::nodes::Node;

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Vec<bool>>,
}

impl Graph {
    fn new(edges: Vec<Vec<bool>>) -> Self {
        Graph { edges }
    }
}

// NOTE: We are doing things in an arena, that is why it is okay not have refrences of edge nodes
// The problem in the general way is that if a node is dropped and it still had an edge that can
// run into undefined behaviour.
// TODO: Keep the above note in mind, and if we want to start dropping nodes, in that case we will
// have to keep references, so that we don't drop nodes that are being used in other graphs. For
// now all deletions should happen at the same time, or rather just let the program complete and
// never delete the nodes.
#[derive(Debug)]
pub struct GraphGroup {
    nodes: Vec<Node>,
    graphs: Vec<Graph>,
}

impl GraphGroup {
    fn new() -> Self {
        GraphGroup {
            nodes: Vec::new(),
            graphs: Vec::new(),
        }
    }

    fn add_nodes(&mut self, nodes: Vec<Node>) {
        self.nodes.extend(nodes);
    }

    fn add_graph(&mut self, edges: Vec<Vec<bool>>) {
        println!("edges length: {}, nodes: {}", edges.len(), self.nodes.len());
        for edge in &edges {
            println!("values: {}", edge.len());
        }
        assert!(
            !edges.is_empty()
                && edges.len() <= self.nodes.len()
                && edges
                    .iter()
                    .all(|x| x.len() == edges.len() && x.len() <= self.nodes.len()),
            "Graph inputs mismatch"
        );
        self.graphs.push(Graph::new(edges));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::nodes::*;

    #[test]
    fn test_graph_working() {
        let mut graph_group = GraphGroup::new();
        //      10
        //    8   9
        //   5  6  7
        //  1  2  3  4

        let mut nodes: Vec<Node> = Vec::new();
        let number_of_nodes = 10;

        for index in 0..=number_of_nodes {
            nodes.push(Node::Operator(OperatorNode::new(format!("{}", index))));
        }

        graph_group.add_nodes(nodes);

        // Right now there is only one graph so it is easy to make the edges
        // but later on, the maker will have to keep the state of arena in mind while making the
        // graph
        let mut edges: Vec<Vec<bool>> = vec![vec![false; number_of_nodes + 1]; number_of_nodes + 1]; // +1 to
        // make it 1 index
        edges[1][5] = true;
        edges[2][5] = true;
        edges[2][6] = true;
        edges[3][6] = true;
        edges[3][7] = true;
        edges[4][7] = true;
        edges[5][8] = true;
        edges[6][8] = true;
        edges[6][9] = true;
        edges[7][9] = true;
        edges[8][10] = true;
        edges[9][10] = true;

        graph_group.add_graph(edges);
        println!("{:?}", graph_group);
    }
}
