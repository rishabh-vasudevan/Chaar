use crate::utils::hash::hash_f32_vec_to_bits;

pub struct Float32 {}
pub struct Float16 {}
pub struct Float8 {}

pub trait DtypeHasher {
    type DtypeValue;

    fn hash_dtype(dtype: &Self::DtypeValue) -> u64;
    fn hash_vec_dtype(dtype_vec: &Vec<Self::DtypeValue>) -> u64;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Dtype {
    Float32,
    Float16,
}

impl Dtype {
    fn data_size(&self) -> usize {
        match self {
            Dtype::Float32 => 4usize,
            Dtype::Float16 => 2usize,
        }
    }

    fn load_data(&self, ptr: *const (), len: usize) {
        let size = self.data_size();
        Self::load_array_unsafe(ptr, size, len);
    }
    fn load_array_unsafe(ptr: *const (), size: usize, len: usize) {
        let bytes_ptr = ptr.cast::<u8>();

        for idx in 0..len {
            let number;
            unsafe {
                let number_bytes = *(bytes_ptr.add(idx * size).cast::<[u8; 4]>());
                number = f32::from_le_bytes(number_bytes);
                // number = f64::from_le_bytes(number_bytes);
            }
            println!("{}", number);
        }
    }

    pub fn hash(&self, data_ptr: *const (), len: usize, size: usize) -> u64 {
        match self {
            Dtype::Float32 => {
                let casted_f32_ptr = data_ptr as *mut f32;
                unsafe { hash_f32_vec_to_bits(casted_f32_ptr, len, size) }
            }
            _ => 1,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Dtype::Float32 => 4,
            Dtype::Float16 => 2,
        }
    }
}

#[cfg(test)]
mod dtype_tests {
    use crate::dtype::Dtype;
}
