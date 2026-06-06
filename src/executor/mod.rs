pub mod cpu;
pub mod cuda;

use crate::graph::compile::ChaarIRS;

#[derive(Debug, Default)]
pub enum ExecutionEnv {
    #[default]
    CPU,
    CUDA,
    METAL,
    HIPA,
}

#[derive(Debug, Default)]
pub struct Executor {
    env: ExecutionEnv,
}

impl Executor {
    pub fn execute(&self, chaar_ir: ChaarIRS) {
        match self.env {
            ExecutionEnv::CPU => cpu::execute::execute(chaar_ir),
            _ => (),
        }
    }
}

#[cfg(test)]
mod executor_tests {

    use crate::{
        dtype::Dtype,
        executor::Executor,
        graph::{graph_group::GraphGroup, nodes::GraphOperator},
        shape_tracker::ShapeTracker,
        tensor::{BufferStore, Tensor},
    };

    #[test]
    fn test_basic() {
        let mut graph_group = GraphGroup::default();

        let graph = graph_group.add_graph(vec![]);

        let mut buffer_store = BufferStore::<f32>::default();

        let a = Tensor::new(
            Some((&mut buffer_store, vec![1.0, 2.0, 3.0, 4.0, 5.0])),
            Dtype::Float32,
            ShapeTracker::new(vec![4], vec![1]),
        );

        let b = Tensor::new(
            Some((&mut buffer_store, vec![5.0, 6.0, 7.0, 8.0, 9.0])),
            Dtype::Float32,
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
        let chaar_ir_one = graph_group.compile(graph);

        let executor = Executor::default();

        executor.execute(chaar_ir_one);
    }
}
