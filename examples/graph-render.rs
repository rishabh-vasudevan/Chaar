use eframe::{
    egui::{self, Context},
    epaint::CubicBezierShape,
};
use egui::{Align2, Color32, FontId, Pos2, Rect, Scene, Sense, Shape, Stroke, Vec2};

struct Node {
    pos: Pos2,
    kind: NodeKind,
    label: String,
    description: String,
}

#[derive(Clone, Copy)]
enum NodeKind {
    Tensor,
    Operator,
    Output,
}

impl NodeKind {
    fn graph_node_for(&self, center: Pos2, size: f32, fill: Color32, stroke: Stroke) -> Shape {
        match self {
            NodeKind::Operator => Shape::Circle(egui::epaint::CircleShape {
                center,
                radius: size,
                fill,
                stroke,
            }),
            NodeKind::Tensor => {
                let rect = Rect::from_center_size(center, Vec2::splat(size * 2.0));
                Shape::Rect(egui::epaint::RectShape::new(
                    rect,
                    4.0,
                    fill,
                    stroke,
                    egui::StrokeKind::Middle,
                ))
            }
            NodeKind::Output => {
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
}

struct GraphVisualizer {
    nodes: Vec<Node>,
    selected: Option<usize>,
    edges: Vec<(usize, usize)>,
    scene_rect: Rect,
}

impl Default for GraphVisualizer {
    fn default() -> Self {
        Self {
            nodes: vec![
                Node {
                    pos: Pos2::new(400.0, 150.0),
                    kind: NodeKind::Tensor,
                    label: "Tensor A".into(),
                    description: "The starting node.".into(),
                },
                Node {
                    pos: Pos2::new(400.0, 300.0),
                    kind: NodeKind::Tensor,
                    label: "Tensor B".into(),
                    description: "The starting node.".into(),
                },
                Node {
                    pos: Pos2::new(800.0, 225.0),
                    kind: NodeKind::Operator,
                    label: "Add".into(),
                    description: "A processing step.".into(),
                },
                Node {
                    pos: Pos2::new(1200.0, 225.0),
                    kind: NodeKind::Output,
                    label: "Output".into(),
                    description: "A decision point.".into(),
                },
            ],
            edges: vec![(0, 2), (1, 2), (2, 3)],
            selected: None,
            scene_rect: Rect::ZERO,
        }
    }
}

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

fn draw_edge(painter: &egui::Painter, from: Pos2, to: Pos2, radius: f32, stroke: Stroke) {
    let dir = (to - from).normalized();
    if !dir.is_finite() {
        return;
    } // same point → skip
    let start = from + dir * radius;
    let end = to - dir * radius;

    let dx = (to.x - from.x) * 0.5;
    let ctrl1 = start + Vec2::new(dx.max(30.0), 0.0);
    let ctrl2 = end - Vec2::new(dx.max(30.0), 0.0);
    // painter.line_segment([start, end], stroke);
    painter.add(CubicBezierShape::from_points_stroke(
        [start, ctrl1, ctrl2, end],
        false,
        Color32::TRANSPARENT,
        stroke,
    ));

    // Arrowhead: two short segments at ±25° from the tip
    let head_len = 10.0;
    let head_half_width = 5.0;
    let perp = Vec2::new(-dir.y, dir.x); // dir rotated 90° CCW
    let base = end - dir * head_len;
    let left = base + perp * head_half_width;
    let right = base - perp * head_half_width;
    painter.add(Shape::convex_polygon(
        vec![end, left, right],
        stroke.color,
        Stroke::NONE,
    ));
}

fn draw_edges(app: &mut GraphVisualizer, ui: &mut egui::Ui, node_size: f32, edge_stroke: Stroke) {
    for &(a, b) in &app.edges {
        draw_edge(
            ui.painter(),
            app.nodes[a].pos,
            app.nodes[b].pos,
            node_size,
            edge_stroke,
        );
    }
}

fn draw_node(app: &mut GraphVisualizer, ctx: &Context, ui: &mut egui::Ui, i: usize, size: f32) {
    let pos = app.nodes[i].pos;
    let rect = Rect::from_center_size(pos, Vec2::splat(size * 2.2));
    let id = ui.id().with(("node", i));
    let response = ui.interact(rect, id, Sense::click_and_drag());

    if response.clicked() {
        if let Some(i) = app.selected {
            app.selected = None;
        } else {
            app.selected = Some(i);
        }
    }
    if response.double_clicked() {
        ctx.memory_mut(|m| m.data.insert_temp(id, true));
    }

    let hovered = response.hovered();
    let selected = Some(i) == app.selected;

    let fill = if selected {
        Color32::from_rgb(255, 200, 80)
    } else if hovered {
        Color32::from_rgb(140, 190, 240)
    } else {
        Color32::from_rgb(100, 150, 200)
    };
    let stroke = Stroke::new(if selected { 3.0 } else { 1.5 }, Color32::WHITE);

    let shape = app.nodes[i].kind.graph_node_for(pos, size, fill, stroke);
    ui.painter().add(shape);
    ui.painter().text(
        pos,
        Align2::CENTER_CENTER,
        &app.nodes[i].label,
        FontId::proportional(13.0),
        Color32::BLACK,
    );

    if hovered {
        response.on_hover_text(&app.nodes[i].description);
    }

    let mut open = ctx.memory(|m| m.data.get_temp::<bool>(id).unwrap_or(false));
    if open {
        egui::Window::new(format!("Edit {}", app.nodes[i].label))
            .id(id.with("modal"))
            .collapsible(false)
            .open(&mut open)
            .show(&ctx, |ui| {
                ui.text_edit_singleline(&mut app.nodes[i].label);
                ui.text_edit_multiline(&mut app.nodes[i].description);
            });
        ctx.memory_mut(|m| m.data.insert_temp(id, open));
    }
}

fn draw_nodes(app: &mut GraphVisualizer, ctx: &Context, ui: &mut egui::Ui, size: f32) {
    for index in 0..app.nodes.len() {
        draw_node(app, ctx, ui, index, size);
    }
}

fn left_panel(ui: &mut egui::Ui, app: &mut GraphVisualizer) {
    egui::Panel::left("details")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Details");
            match app.selected {
                Some(i) => {
                    let n = &app.nodes[i];
                    ui.label(format!("Name: {}", n.label));
                    ui.label(format!("Kind: {:?}", n.kind as u8));
                    ui.separator();
                    ui.label(&n.description);
                }
                None => {
                    ui.label("Click a shape to inspect it.");
                }
            }
        });
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "shapes",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GraphVisualizer::default()))),
    )
}
