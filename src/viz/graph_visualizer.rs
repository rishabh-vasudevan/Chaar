use crate::{
    graph::graph::GraphGroup,
    viz::{
        nodes::{VizNode, VizNodeKind},
        ui_elements::*,
    },
};
use eframe::egui;
use egui::{Color32, Pos2, Rect, Scene, Sense, Stroke, Vec2};

pub struct GraphVisualizer {
    pub nodes: Vec<VizNode>,
    pub selected: Option<usize>,
    pub edges: Vec<(usize, usize)>,
    pub scene_rect: Rect,
}

impl Default for GraphVisualizer {
    fn default() -> Self {
        Self {
            nodes: vec![
                VizNode::new(
                    Pos2::new(400.0, 150.0),
                    VizNodeKind::Tensor,
                    "Tensor A".into(),
                    "The starting node.".into(),
                ),
                VizNode::new(
                    Pos2::new(400.0, 300.0),
                    VizNodeKind::Tensor,
                    "Tensor B".into(),
                    "The starting node.".into(),
                ),
                VizNode::new(
                    Pos2::new(800.0, 225.0),
                    VizNodeKind::Operator,
                    "Add".into(),
                    "A processing step.".into(),
                ),
                VizNode::new(
                    Pos2::new(1200.0, 225.0),
                    VizNodeKind::Output,
                    "Output".into(),
                    "A decision point.".into(),
                ),
            ],
            edges: vec![(0, 2), (1, 2), (2, 3)],
            selected: None,
            scene_rect: Rect::ZERO,
        }
    }
}

pub const HORIZONTAL_SPACING: f32 = 250.0;
pub const MIN_VERTICAL_SPACING: f32 = 150.0;
pub const PADDING: f32 = 100.0;

// impl GraphVisualizer {
//     fn new(graph: GraphGroup, graph_index: usize) -> Self {
//         let topo_sorted = graph.topo_sort(graph_index);
//         GraphVisualizer {
//             nodes: graph
//                 .get_nodes()
//                 .clone()
//                 .iter()
//                 .map(|node| {
//                     VizNode::new(
//                         node.pos,
//                         VizNodeKind::get_node_kind_from_node(&node),
//                         node.get_label(),
//                         "Random description".to_string(),
//                     )
//                 })
//                 .collect(),
//             selected: None,
//             edges: graph.get_edges(graph_index).clone(),
//             scene_rect: Rect::ZERO,
//         }
//     }
//
//     fn compute_positions(
//         topo_sorted: &Vec<TopoSorted>,
//         edges: &Vec<(usize, usize), num_nodes: usize>,
//     ) -> Vec<Pos2> {
//     }
// }
//
impl eframe::App for GraphVisualizer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        left_panel(ui, self);

        let size_left = ui.available_size();

        let scene = Scene::new()
            .max_inner_size(Vec2::new(size_left.x * 2.0, size_left.y * 2.0))
            .zoom_range(0.1..=f32::INFINITY);

        // FIXME: take copy here and then update it back later after the show block does not seem
        // like a clean way to do this
        let mut scene_rect = self.scene_rect;
        scene.show(ui, &mut scene_rect, |ui| {
            let full_rect = ui.available_rect_before_wrap();
            ui.allocate_rect(full_rect, Sense::hover());

            let node_size = 35.0;

            let edge_stroke = Stroke::new(1.8, Color32::from_gray(180));
            draw_edges(self, ui, node_size, edge_stroke);

            draw_nodes(self, &ctx, ui, node_size);
        });

        self.scene_rect = scene_rect;
    }
}
