use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_f32_to_bits(data: f32) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.to_bits().hash(&mut hasher);
    hasher.finish()
}

pub unsafe fn hash_f32_vec_to_bits(data: *mut f32, len: usize, size: usize) -> u64 {
    let vec_f32;
    unsafe {
        vec_f32 = Vec::from_raw_parts(data, len, size);
    }
    let hashes = vec_f32
        .iter()
        .map(|val| hash_f32_to_bits(*val))
        .collect::<Vec<u64>>();

    let mut hasher = DefaultHasher::new();
    hashes.hash(&mut hasher);
    hasher.finish()
}
