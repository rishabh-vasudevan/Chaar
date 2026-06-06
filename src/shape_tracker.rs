use std::cmp;

#[derive(Debug, Clone)]
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

        assert!(
            shape
                .iter()
                .enumerate()
                .any(|(dim_idx, dim)| dim % stride[dim_idx] == 0)
        );
        ShapeTracker { shape, stride }
    }

    pub fn check_valid_shape<T>(&self, values: &Vec<T>) -> bool {
        let expected_values: usize = self.shape.iter().product();
        expected_values == values.len()
    }

    pub fn is_equal(&self, other: ShapeTracker) -> bool {
        self.shape == other.shape && self.stride == other.stride
    }

    pub fn get_size(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn index_mem_offset(&self, idx: &Vec<usize>) -> usize {
        idx.iter()
            .zip(self.stride.iter())
            .map(|(idx, stride)| idx * stride)
            .sum()
    }
}

impl IntoIterator for ShapeTracker {
    type Item = Vec<usize>;
    type IntoIter = ShapeTrackerItr;

    fn into_iter(self) -> Self::IntoIter {
        ShapeTrackerItr::new(self.shape)
    }
}

#[derive(Debug, Clone)]
pub struct ShapeTrackerItr {
    current: Vec<usize>,
    dim: Vec<usize>,
    done: bool,
}

impl ShapeTrackerItr {
    fn new(dim: Vec<usize>) -> Self {
        let done = dim.contains(&0);

        ShapeTrackerItr {
            current: vec![0; dim.len()],
            dim,
            done,
        }
    }
}

impl Iterator for ShapeTrackerItr {
    type Item = Vec<usize>;

    // TODO: take into consideration the stride as well
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current_clone = self.current.clone();

        for dim_pos in (0..self.dim.len()).rev() {
            self.current[dim_pos] += 1;

            if self.current[dim_pos] < self.dim[dim_pos] {
                return Some(current_clone);
            }

            self.current[dim_pos] = 0;
        }

        self.done = true;
        Some(current_clone)
    }
}

#[cfg(test)]
mod shape_tracker_tests {
    use super::*;

    #[test]
    fn test_shape_tracker_init() {
        let shape_tracker = ShapeTracker::new(vec![64, 3], vec![2, 3]);
        println!("{:?} {:?}", shape_tracker.shape, shape_tracker.stride);
    }

    #[test]
    fn test_shape_tracker_iterator() {
        let shape_tracker = ShapeTracker::new(vec![3, 4, 2, 3], vec![1, 1, 2, 1]);

        for dims in shape_tracker {
            println!("{:?}", dims);
        }
    }
}
