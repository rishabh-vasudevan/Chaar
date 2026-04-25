pub struct Float32 {}
pub struct Float16 {}
pub struct Float8 {}

#[derive(Debug)]
pub enum Dtype {
    Float32(Vec<f32>),
}

pub enum DtypeDuplicate {
    Float32,
    Float16,
}

#[cfg(test)]
mod dtype_tests {

    #[test]
    fn test_load_data() {
        let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let (mut ptr, len, cap) = data.into_raw_parts();

        unsafe {
            for i in 0..len {
                ptr.add(i).write(*ptr.add(i) + i as f64)
            }
            let rebuilt = Vec::from_raw_parts(ptr, len, cap);
            println!("{:?}", rebuilt);
        }
    }
}
