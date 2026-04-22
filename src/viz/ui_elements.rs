use crate::viz::graph_visualizer::GraphVisualizer;
use eframe::egui::{self, Context};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, Vec2};

pub fn draw_edge(painter: &egui::Painter, from: Pos2, to: Pos2, radius: f32, stroke: Stroke) {
    let dir = (to - from).normalized();
    if !dir.is_finite() {
        return;
    } // same point → skip
    let start = from + dir * radius;
    let end = to - dir * radius;
    painter.line_segment([start, end], stroke);

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

pub fn draw_edges(
    app: &mut GraphVisualizer,
    ui: &mut egui::Ui,
    node_size: f32,
    edge_stroke: Stroke,
) {
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

pub fn draw_node(app: &mut GraphVisualizer, ctx: &Context, ui: &mut egui::Ui, i: usize, size: f32) {
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

pub fn draw_nodes(app: &mut GraphVisualizer, ctx: &Context, ui: &mut egui::Ui, size: f32) {
    for index in 0..app.nodes.len() {
        draw_node(app, ctx, ui, index, size);
    }
}

pub fn left_panel(ui: &mut egui::Ui, app: &mut GraphVisualizer) {
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
