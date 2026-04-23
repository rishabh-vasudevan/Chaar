use crate::graph::graph_group::GraphGroup;

#[derive(Debug)]
pub struct ChaarIROne {}

#[derive(Debug)]
pub struct ChaarIRTwo {}

pub enum ChaarIRS {
    ChaarIROne(ChaarIROne),
    ChaarIRTwo(ChaarIRTwo),
}

impl ChaarIRS {
    pub fn search_and_compile(graph_group: &GraphGroup) {
        todo!()
    }
    pub fn compile(graph_group: &GraphGroup, graph_index: usize) -> ChaarIRS {
        let chaar_ir_one = ChaarIRS::level_one(graph_group, graph_index);
        ChaarIRS::level_two(chaar_ir_one)
    }
    fn level_one(graph_group: &GraphGroup, graph_index: usize) -> ChaarIROne {
        // do first level of compilation
        println!("Reached Level One");
        ChaarIROne {}
    }

    fn level_two(chaar_ir_one: ChaarIROne) -> ChaarIRS {
        println!("Reached Level Two");
        ChaarIRS::ChaarIRTwo(ChaarIRTwo {})
    }
}
