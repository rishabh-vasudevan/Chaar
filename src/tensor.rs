use crate::dtype::Dtype;
use crate::shape_tracker::ShapeTracker;

#[derive(Debug)]
struct MemBufferData {
    ptr: *const (),
    len: usize,
}

#[derive(Debug)]
enum TensorBuffer {
    Mem(MemBufferData),
    File(String), //path
}

#[derive(Debug)]
pub struct Tensor {
    buffer: TensorBuffer,
    dtype_data: Dtype,
    shape: ShapeTracker,
}

impl Tensor {
    pub fn new(buffer: TensorBuffer, dtype_data: Dtype, shape: ShapeTracker) -> Self {
        // match &dtype_data {
        //     Dtype::Float32(data) => {
        //         assert!(shape.check_valid_shape(data));
        //     }
        // }

        Tensor {
            buffer,
            dtype_data,
            shape,
        }
    }

    pub fn tolist<T>(&self) -> Vec<T>
    where
        T: std::fmt::Debug + Clone + Copy,
    {
        match &self.buffer {
            TensorBuffer::Mem(mem_buffer_data) => {
                let data_ptr = mem_buffer_data.ptr.cast::<T>();
                let mut_ptr = data_ptr as *mut T;
                let mut number_vec = vec![];
                for idx in 0..mem_buffer_data.len {
                    let number;
                    unsafe {
                        number = *mut_ptr.add(idx);
                    }
                    number_vec.push(number);
                }
                number_vec
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
        let (buffer_ptr, len, _size) = buffer.into_raw_parts();

        let tensor_buffer = TensorBuffer::Mem(MemBufferData {
            ptr: buffer_ptr as *const (),
            len,
        });

        let new_tensor = Tensor::new(
            tensor_buffer,
            Dtype::Float32,
            ShapeTracker::new(vec![2, 3], vec![3, 1]),
        );
        let returned_list = new_tensor.tolist::<f32>();
        println!("{:?}", returned_list);
    }
}
