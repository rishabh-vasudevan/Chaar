use eframe::egui::{self, Color32, Pos2, Rect, Shape, Stroke, Vec2};

use crate::graph::nodes::Node;

pub struct VizNode {
    pub pos: Pos2,
    pub kind: VizNodeKind,
    pub label: String,
    pub description: String,
}

impl VizNode {
    pub fn new(pos: Pos2, kind: VizNodeKind, label: String, description: String) -> Self {
        VizNode {
            pos,
            kind,
            label,
            description,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VizNodeKind {
    Tensor,
    Operator,
    Output,
    Buffer,
}

impl VizNodeKind {
    pub fn graph_node_for(&self, center: Pos2, size: f32, fill: Color32, stroke: Stroke) -> Shape {
        match self {
            VizNodeKind::Operator => Shape::Circle(egui::epaint::CircleShape {
                center,
                radius: size,
                fill,
                stroke,
            }),
            VizNodeKind::Tensor => {
                let rect = Rect::from_center_size(center, Vec2::splat(size * 2.0));
                Shape::Rect(egui::epaint::RectShape::new(
                    rect,
                    4.0,
                    fill,
                    stroke,
                    egui::StrokeKind::Middle,
                ))
            }
            VizNodeKind::Buffer => {
                let rect = Rect::from_center_size(center, Vec2::splat(size * 2.0));
                Shape::Rect(egui::epaint::RectShape::new(
                    rect,
                    4.0,
                    fill,
                    stroke,
                    egui::StrokeKind::Middle,
                ))
            }

            VizNodeKind::Output => {
                let pts = vec![
                    Pos2::new(center.x, center.y - size),
                    Pos2::new(center.x + size, center.y),
                    Pos2::new(center.x, center.y + size),
                    Pos2::new(center.x - size, center.y),
                ];
                Shape::convex_polygon(pts, fill, stroke)
            }
        }
    }

    pub fn get_node_kind_from_node(node: &Node) -> VizNodeKind {
        match node {
            Node::Tensor(_) => VizNodeKind::Tensor,
            Node::Operator(_) => VizNodeKind::Operator,
            Node::Buffer(_) => VizNodeKind::Operator,
            Node::Output => VizNodeKind::Output,
        }
    }
}
