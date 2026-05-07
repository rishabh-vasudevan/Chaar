use crate::graph::compile::ChaarIROne;

#[derive(Debug, Default)]
pub enum ExecutionEnv {
    #[default]
    CPU,
    CUDA,
    METAL,
    HIPA,
}

#[derive(Debug, Default)]
pub struct Executor {
    env: ExecutionEnv,
}

impl Executor {
    pub fn execute(chaar_ir: ChaarIROne) {
        // chaar_ir.kernels.iter().for_each(|kernel| {
        // })
        todo!()
    }
}
