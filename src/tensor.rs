use crate::dtype::Dtype;
use crate::shape_tracker::ShapeTracker;

#[derive(Debug)]
pub struct Tensor {
    dtype_data: Dtype,
    shape: ShapeTracker,
}

impl Tensor {
    pub fn new(dtype_data: Dtype, shape: ShapeTracker) -> Self {
        match &dtype_data {
            Dtype::Float32(data) => {
                assert!(shape.check_valid_shape(data));
            }
        }
        Tensor { dtype_data, shape }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::Dtype;
    use crate::shape_tracker::ShapeTracker;

    #[test]
    fn test_new_vector() {
        let new_tensor = Tensor::new(
            Dtype::Float32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            ShapeTracker::new(vec![2, 3], vec![3, 1]),
        );
        println!("{:?}", new_tensor);
    }
}
