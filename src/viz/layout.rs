use std::{cmp::max, collections::HashMap, ops::Div};

use crate::graph::graph_group::TopoSorted;
use eframe::egui::Pos2;

pub const HORIZONTAL_SPACING: f32 = 250.0;
pub const MIN_VERTICAL_SPACING: f32 = 150.0;
pub const PADDING: f32 = 100.0;

pub fn compute_positions(topo_sorted: &Vec<TopoSorted>) -> Vec<Pos2> {
    let mut group_by_layer: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut max_layer = 0;
    for topo_node in topo_sorted {
        max_layer = max(max_layer, topo_node.layer);
        group_by_layer
            .entry(topo_node.layer)
            .or_default()
            .push(topo_node.node_id);
    }

    let mut x_pos = vec![0.0f32; topo_sorted.len()];
    let mut y_pos = vec![0.0f32; topo_sorted.len()];
    for layer in 0..=max_layer {
        for node in group_by_layer.get(&layer).unwrap() {
            x_pos[*node] = (layer as f32) * HORIZONTAL_SPACING;

            if (layer == 0) {
                let num_of_nodes_in_layer = group_by_layer.get(&layer).unwrap().len();
                let total_height = MIN_VERTICAL_SPACING * num_of_nodes_in_layer as f32;
                let start_y = -total_height / 2.0;

                for (idx, node) in group_by_layer.get(&layer).unwrap().iter().enumerate() {
                    y_pos[*node] = start_y + (idx as f32 * MIN_VERTICAL_SPACING);
                }
            } else {
                let node_parents = &topo_sorted[*node].parents;
                y_pos[*node] = node_parents
                    .iter()
                    .map(|parent| y_pos[*parent])
                    .sum::<f32>()
                    .div(node_parents.len() as f32);
            }
        }
    }

    for layer in 1..=max_layer {
        let mut nodes_in_layer = group_by_layer.get(&layer).unwrap().clone();
        nodes_in_layer.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        for (idx, node) in nodes_in_layer.iter().enumerate() {
            y_pos[*node] = if idx == 0 {
                y_pos[*node]
            } else {
                y_pos[*node].max(y_pos[*node - 1] + MIN_VERTICAL_SPACING)
            }
        }
    }

    x_pos
        .iter()
        .zip(y_pos)
        .map(|(x, y)| Pos2::new(*x, y))
        .collect()
}

pub fn center_positions(positions: &mut [Pos2]) {
    if positions.is_empty() {
        return;
    }

    let (min_x, max_x, min_y, max_y) = calculate_min_max_coordinates(positions);

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    // Offset all positions so center is at (0, 0)
    for pos in positions.iter_mut() {
        pos.x -= center_x;
        pos.y -= center_y;
    }
}

pub fn compute_scene_rect(positions: &[Pos2]) -> eframe::egui::Rect {
    if positions.is_empty() {
        return eframe::egui::Rect::ZERO;
    }

    let (min_x, max_x, min_y, max_y) = calculate_min_max_coordinates(positions);

    eframe::egui::Rect::from_min_max(
        eframe::egui::Pos2::new(min_x - PADDING, min_y - PADDING),
        eframe::egui::Pos2::new(max_x + PADDING, max_y + PADDING),
    )
}

fn calculate_min_max_coordinates(positions: &[Pos2]) -> (f32, f32, f32, f32) {
    let mut min_x = positions[0].x;
    let mut max_x = positions[0].x;
    let mut min_y = positions[0].y;
    let mut max_y = positions[0].y;

    for pos in positions.iter().skip(1) {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }
    (min_x, max_x, min_y, max_y)
}
