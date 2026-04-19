use crate::dtype::Dtype;

#[derive(Debug)]
pub struct ShapeTracker {
    // TODO: remove pub
    pub shape: Vec<usize>,
    pub stride: Vec<usize>,
}

impl ShapeTracker {
    pub fn new(shape: Vec<usize>, stride: Vec<usize>) -> Self {
        assert!(
            shape.len() == stride.len(),
            "Number of values in shape and stride should be equal"
        );
        ShapeTracker { shape, stride }
    }

    pub fn check_valid_shape<T>(&self, values: &Vec<T>) -> bool {
        let expected_values: usize = self.shape.iter().product();
        expected_values == values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [[1, 2 ,3], [1, 2 ,3]]
    /// Shape: 2, 3 and Stride: 1, 2
    #[test]
    fn test_shape_tracker_init() {
        let shape_tracker = ShapeTracker::new(vec![2, 3], vec![1, 2]);
        println!("{:?} {:?}", shape_tracker.shape, shape_tracker.stride);
    }
}
