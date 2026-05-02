use std::os::raw::c_void;

pub struct Float32 {}
pub struct Float16 {}
pub struct Float8 {}

#[derive(Debug)]
pub enum Dtype {
    Float32(Vec<f32>),
}

pub enum DtypeDuplicate {
    Float32New,
    Float16New,
}

impl DtypeDuplicate {
    fn data_size(&self) -> usize {
        match self {
            DtypeDuplicate::Float32New => 4usize,
            DtypeDuplicate::Float16New => 2usize,
        }
    }

    fn load_data(&self, ptr: *const c_void, len: usize) {
        let size = self.data_size();
        Self::load_array_unsafe(ptr, size, len);
    }
    fn load_array_unsafe(ptr: *const c_void, size: usize, len: usize) {
        let bytes_ptr = ptr.cast::<u8>();
        for idx in 0..len {
            let number;
            unsafe {
                let number_bytes = *(bytes_ptr.add(idx * size).cast::<[u8; 4]>());
                number = f32::from_le_bytes(number_bytes);
            }
            println!("{}", number);
        }
    }
}

pub struct DtypeTest {
    dtype: DtypeDuplicate,
}

impl DtypeTest {
    fn new(dtype: DtypeDuplicate) -> Self {
        DtypeTest { dtype }
    }
    fn load_data(&self, ptr: *const c_void, len: usize) {
        self.dtype.load_data(ptr, len);
    }
}

#[cfg(test)]
mod dtype_tests {
    use crate::dtype::{DtypeDuplicate, DtypeTest};
    use std::os::raw::c_void;

    #[test]
    fn test_load_data() {
        let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let (ptr, len, cap) = data.into_raw_parts();

        let mut_const_ptr = ptr.cast::<*const c_void>();
        let const_ptr = mut_const_ptr as *const c_void;

        let dtypeTest = DtypeTest::new(DtypeDuplicate::Float32New);

        unsafe {
            dtypeTest.load_data(const_ptr, len);
        }

        // unsafe {
        //     for i in 0..len {
        //         // ptr.add(i).write(*ptr.add(i) + i as f64)
        //         const_ptr.map_addr(|addr| );
        //     }
        //     let rebuilt = Vec::from_raw_parts(ptr, len, cap);
        //     println!("{:?}", rebuilt);
        // }
    }
}
