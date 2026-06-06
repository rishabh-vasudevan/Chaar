use crate::{
    graph::{
        compile::{ChaarIRS, KernelOperator},
        graph_group::{GraphGroup, NodeIdx},
    },
    tensor::Tensor,
};
use std::{collections::HashMap, mem};

#[derive(Debug)]
struct AllotedBuffer {
    tensor: Tensor,
    size: usize,
}

impl AllotedBuffer {
    fn new(empty_tensor: Tensor, size: usize) -> Self {
        AllotedBuffer {
            tensor: empty_tensor,
            size,
        }
    }
}

struct Executor {}

impl Executor {
    fn execute_binary_op(
        graph_group: &GraphGroup,
        operator: KernelOperator,
        input_one: NodeIdx,
        input_two: NodeIdx,
        output_node: NodeIdx,
        alloted_buffers: &mut HashMap<NodeIdx, AllotedBuffer>,
    ) {
        match operator {
            KernelOperator::Add => {
                let tensor_one = if alloted_buffers.contains_key(&input_one) {
                    alloted_buffers.get(&input_one).unwrap().tensor.clone()
                } else {
                    graph_group
                        .get_node(input_one)
                        .get_tensor()
                        .expect("did not get tensor")
                };
                let tensor_two = if alloted_buffers.contains_key(&input_two) {
                    alloted_buffers.get(&input_two).unwrap().tensor.clone()
                } else {
                    graph_group
                        .get_node(input_two)
                        .get_tensor()
                        .expect("did not get tensor")
                };
                let output_tensor = graph_group.get_node(output_node).get_buffer();
                dbg!(&operator);
                dbg!(&tensor_one);
                dbg!(&tensor_two);

                assert!(tensor_one.shape.is_equal(tensor_two.shape));
            }
            KernelOperator::MatMul => {}
            KernelOperator::NoOp => {}
        }
    }
}

pub fn execute(chaar_ir: ChaarIRS) {
    let chaar_ir_one = if let ChaarIRS::ChaarIROne(chaar_ir_one) = chaar_ir {
        chaar_ir_one
    } else {
        panic!("Execution is not supported for this IR");
    };

    let mut alloted_buffer_hash_map: HashMap<NodeIdx, AllotedBuffer> = HashMap::new();

    chaar_ir_one
        .buffers_to_create
        .iter()
        .for_each(|buffer_to_create| {
            let box_size =
                buffer_to_create.shape_tracker.get_size() * buffer_to_create.dtype.get_size();
            let empty_tensor_buffer = Tensor::empty_new(
                buffer_to_create.dtype.clone(),
                buffer_to_create.shape_tracker.clone(),
            );
            alloted_buffer_hash_map.insert(
                buffer_to_create.buffer_id,
                AllotedBuffer::new(empty_tensor_buffer, box_size),
            );
        });

    let graph_group = &chaar_ir_one.graph_group;

    chaar_ir_one.kernels.iter().for_each(|kernel| {
        let operator = kernel.operator.clone();
        Executor::execute_binary_op(
            graph_group,
            operator,
            kernel.parents[0],
            kernel.parents[1],
            kernel.output,
            &mut alloted_buffer_hash_map,
        );
    });
    // Start Compute
    //      Check conditions for operation (This should be done before actually sending compute to
    //      the device as we want better happy path in the device hot path)
    //      Perform operation
    //  Complete
    //  Output Result
}
