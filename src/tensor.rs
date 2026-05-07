use std::collections::HashMap;

use crate::dtype::Dtype;
use crate::shape_tracker::ShapeTracker;

type HashType = u64;

#[derive(Debug, Default)]
pub struct BufferStore<T> {
    data_hashmap: HashMap<HashType, Vec<T>>,
}

impl<T> BufferStore<T> {
    fn add(&mut self, data: Vec<T>, dtype: &Dtype) -> u64
    where
        T: Clone,
    {
        let data_clone = data.clone();
        let (ptr, len, size) = data.into_raw_parts();
        let casted_to_opaque = ptr as *const ();
        let hash = dtype.hash(casted_to_opaque, len, size);

        self.data_hashmap.insert(hash, data_clone);
        hash
    }

    fn get_buffer_data(&self, hash: u64) -> Option<&Vec<T>>
    where
        T: std::fmt::Debug + Clone,
    {
        self.data_hashmap.get(&hash)
    }
}

#[derive(Debug)]
enum TensorBuffer {
    Mem(HashType),
    File(String), //path
}

#[derive(Debug)]
pub struct Tensor {
    pub buffer: Option<TensorBuffer>,
    pub dtype_data: Dtype,
    pub shape: ShapeTracker,
}

impl Tensor {
    pub fn new<T>(
        buffer_req: Option<(&mut BufferStore<T>, Vec<T>)>,
        dtype_data: Dtype,
        shape: ShapeTracker,
    ) -> Self
    where
        T: std::fmt::Debug + Clone,
    {
        // match &dtype_data {
        //     Dtype::Float32(data) => {
        //         assert!(shape.check_valid_shape(data));
        //     }
        // }

        let tensor_buffer_hash;
        let mut tensor_buffer: Option<TensorBuffer> = None;

        if let Some((buffer_store, data)) = buffer_req {
            tensor_buffer_hash = buffer_store.add(data, &dtype_data);
            tensor_buffer = Some(TensorBuffer::Mem(tensor_buffer_hash));
        }

        Tensor {
            buffer: tensor_buffer,
            dtype_data,
            shape,
        }
    }

    pub fn tolist<T>(&self, buffer_store: &BufferStore<T>) -> Vec<T>
    where
        T: std::fmt::Debug + Clone + Copy,
    {
        match &self.buffer {
            Some(TensorBuffer::Mem(buffer_hash)) => {
                buffer_store.get_buffer_data(*buffer_hash).unwrap().clone()
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tensor_tests {
    use super::*;
    use crate::dtype::Dtype;
    use crate::shape_tracker::ShapeTracker;

    #[test]
    fn test_new_vector() {
        let buffer: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let buffer_clone = buffer.clone();

        let mut buffer_store = BufferStore::<f32>::default();

        let new_tensor = Tensor::new(
            Some((&mut buffer_store, buffer)),
            Dtype::Float32,
            ShapeTracker::new(vec![2, 3], vec![3, 1]),
        );
        let returned_list = new_tensor.tolist::<f32>(&buffer_store);
        assert!(returned_list == buffer_clone);
    }

    #[test]
    fn test_buffer_same_hash() {
        let mut buffer_store = BufferStore::<f32>::default();
        let test_vector_one = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let test_vector_two = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        buffer_store.add(test_vector_one, &Dtype::Float32);
        buffer_store.add(test_vector_two, &Dtype::Float32);
        assert!(buffer_store.data_hashmap.len() == 1);
    }

    #[test]
    fn test_buffer_different_hash() {
        let mut buffer_store = BufferStore::<f32>::default();
        let test_vector_one = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let test_vector_two = vec![1.0, 2.0, 3.0, 4.0, 5.0, 7.0];

        buffer_store.add(test_vector_one, &Dtype::Float32);
        buffer_store.add(test_vector_two, &Dtype::Float32);

        assert!(buffer_store.data_hashmap.len() == 2);
    }
}
