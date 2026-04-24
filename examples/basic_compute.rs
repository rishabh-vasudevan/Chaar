use chaar::{
    dtype::Dtype,
    graph::{graph_group::GraphGroup, nodes::GraphOperator},
    shape_tracker::ShapeTracker,
    tensor::Tensor,
    utils::parse_args::parse_args,
    viz::graph_visualizer::GraphVisualizer,
};

fn main() {
    let args = parse_args();
    let mut graph_group = GraphGroup::default();

    let graph = graph_group.add_graph(vec![]);

    let a = Tensor::new(
        Dtype::Float32(vec![1.0, 2.0, 3.0, 4.0]),
        ShapeTracker::new(vec![4], vec![1]),
    );

    let b = Tensor::new(
        Dtype::Float32(vec![5.0, 6.0, 7.0, 8.0]),
        ShapeTracker::new(vec![4], vec![1]),
    );

    //graph_append!(a, A)
    let a = graph_group.add_tensor(a, stringify!(a).to_string());
    let b = graph_group.add_tensor(b, stringify!(b).to_string());

    let c = graph_group.add_operator(
        graph,
        GraphOperator::Add,
        vec![a, b],
        stringify!(c).to_string(),
    );

    let d = graph_group.add_operator(
        graph,
        GraphOperator::Add,
        vec![c, a],
        stringify!(d).to_string(),
    );

    let d = graph_group.add_operator(
        graph,
        GraphOperator::MatMul,
        vec![d, b],
        stringify!(e).to_string(),
    );

    // graph_group.search_and_compile(graph);
    graph_group.compile(graph);

    if args.contains_key("viz") {
        eframe::run_native(
            "shapes",
            eframe::NativeOptions::default(),
            Box::new(|_| Ok(Box::new(GraphVisualizer::new(graph_group, graph)))),
        )
        .unwrap();
    }
}

/*
*
* Possible Option: So this would work cause the tensor only needs to be added to a graph
* only when it is being computed, as it is if it is worthless it will get optimzed out
*
* let A = tensor!()
* let B = tensor!()
*
* let c = add!(A, B)
*
* let d = add!(c, a)
*
*
*
*
*
* mul!(A,B)
* mat_mul!(A,B)
*
*/
