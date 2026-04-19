pub struct Float32 {}
pub struct Float16 {}
pub struct Float8 {}

#[derive(Debug)]
pub enum Dtype {
    Float32(Vec<f32>),
}
