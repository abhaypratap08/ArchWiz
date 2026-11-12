use crate::graph::{Graph, GraphNodeKind};
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

pub const CARD_HEIGHT: usize = 4;
const V_GAP: usize = 2;
const ROW_GAP: usize = 2;
const H_GAP: usize = 3;
const MIN_TEXT_WIDTH: usize = 12;
const MARGIN: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct NodeRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub x: usize,
    pub y: usize,
    pub ch: char,
    pub owner: usize,
}

#[derive(Debug)]
pub struct Layout {
    pub width: usize,
    pub height: usize,
    pub nodes: Vec<Option<NodeRect>>,
    pub strokes: Vec<Stroke>,
    pub preview_limit: usize,
}

#[allow(dead_code)]
pub fn compute(graph: &Graph, terminal_width: usize) -> Layout {
    compute_with(graph, terminal_width, 3)
}

pub fn compute_with(graph: &Graph, terminal_width: usize, preview_limit: usize) -> Layout {
    let available = terminal_width.saturating_sub(MARGIN * 2).max(20);
    let card_widths: Vec<usize> = graph.nodes.iter().map(card_width).collect();
    let node_heights: Vec<usize> = graph
        .nodes
        .iter()
        .map(|n| node_height(n, preview_limit))
        .collect();
    let children = build_children(graph);

    let mut nodes = vec![None; graph.nodes.len()];
    let mut strokes = Vec::new();

    if graph.nodes.is_empty() {
        return Layout {
            width: 0,
            height: 0,
            nodes,
            strokes,
            preview_limit,
        };
    }

    let root = graph
        .nodes
        .iter()
        .find(|n| matches!(n.kind, GraphNodeKind::Project))
        .map(|n| n.id)
        .unwrap_or(0);

    // Cache each subtree's dimensions once (post-order; child ids are greater than parent ids).
    let mut dims = vec![(0usize, 0usize); graph.nodes.len()];
    for id in (0..graph.nodes.len()).rev() {
        let cw = card_widths[id].min(available).max(10);
        let ch = node_heights[id];
        let child_ids = &children[id];
        if child_ids.is_empty() {
            dims[id] = (cw, ch);
            continue;
        }
        let child_dims: Vec<(usize, usize)> = child_ids.iter().map(|&c| dims[c]).collect();
        let rows = wrap_rows(child_ids, &child_dims, available);

        let mut max_row_width = 0usize;
        let mut total_height = ch + V_GAP;
        let mut first = true;
        for row in &rows {
            let row_width: usize = row.iter().map(|&i| child_dims[i].0).sum::<usize>()
                + H_GAP * row.len().saturating_sub(1);
            if row_width > max_row_width {
                max_row_width = row_width;
            }
            let row_height = row
                .iter()
                .map(|&i| child_dims[i].1)
                .max()
                .unwrap_or(CARD_HEIGHT);
            if !first {
                total_height += ROW_GAP;
            }
            first = false;
            total_height += row_height;
        }

        let total_width = cw.max(max_row_width).min(available);
        dims[id] = (total_width, total_height);
    }

    let (total_w, total_h) = dims[root];
    let offset = terminal_width.saturating_sub(total_w) / 2;

    place(
        root,
        offset,
        0,
        available,
        &card_widths,
        &node_heights,
        &children,
        &dims,
        &mut nodes,
        &mut strokes,
    );

    let strokes = merge_strokes(strokes);

    Layout {
        width: terminal_width,
        height: total_h,
        nodes,
        strokes,
        preview_limit,
    }
}

fn build_children(graph: &Graph) -> Vec<Vec<usize>> {
    let mut children = vec![Vec::new(); graph.nodes.len()];
    for edge in &graph.edges {
        if edge.from >= graph.nodes.len() || edge.to >= graph.nodes.len() || edge.from == edge.to {
            continue;
        }
        children[edge.from].push(edge.to);
    }
    children
}

fn node_height(node: &crate::graph::GraphNode, preview_limit: usize) -> usize {
    if node.details.is_empty() {
        CARD_HEIGHT
    } else {
        let shown = node.details.len().min(preview_limit);
        let extra = if node.details.len() > preview_limit {
            1
        } else {
            0
        };
        3 + shown + extra
    }
}

fn card_width(node: &crate::graph::GraphNode) -> usize {
    let mut content_width = node.label.width().max(node.subtitle.width());
    for d in &node.details {
        content_width = content_width.max(d.width());
    }
    let content_width = content_width.max(MIN_TEXT_WIDTH);
    (content_width + 4).min(80)
}

fn wrap_rows(child_ids: &[usize], dims: &[(usize, usize)], available: usize) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_width = 0usize;

    for (idx, _) in child_ids.iter().enumerate() {
        let w = dims[idx].0;
        let needed = if current.is_empty() {
            w
        } else {
            current_width + H_GAP + w
        };
        if !current.is_empty() && needed > available {
            rows.push(std::mem::take(&mut current));
            current_width = 0;
        }
        if current.is_empty() {
            current_width = w;
        } else {
            current_width += H_GAP + w;
        }
        current.push(idx);
    }
    if !current.is_empty() {
        rows.push(current);
    }
    if rows.is_empty() {
        rows.push(Vec::new());
    }
    rows
}

#[allow(clippy::too_many_arguments)]
fn place(
    id: usize,
    origin_x: usize,
    origin_y: usize,
    available: usize,
    card_widths: &[usize],
    node_heights: &[usize],
    children: &[Vec<usize>],
    dims: &[(usize, usize)],
    nodes: &mut [Option<NodeRect>],
    strokes: &mut Vec<Stroke>,
) {
    let cw = card_widths[id].min(available).max(10);
    let ch = node_heights[id];
    let (block_w, _) = dims[id];

    let card_x = origin_x + (block_w.saturating_sub(cw)) / 2;
    let card_y = origin_y;
    nodes[id] = Some(NodeRect {
        x: card_x,
        y: card_y,
        w: cw,
        h: ch,
    });

    let child_ids = &children[id];
    if child_ids.is_empty() {
        return;
    }

    let child_dims: Vec<(usize, usize)> = child_ids.iter().map(|&c| dims[c]).collect();

    let rows = wrap_rows(child_ids, &child_dims, available);

    let parent_center_x = card_x + cw / 2;
    let parent_bottom_y = card_y + ch - 1;

    // Compute row origins.
    let mut row_origins: Vec<(usize, usize)> = Vec::new();
    let mut y_cursor = origin_y + ch + V_GAP;
    let mut first_row = true;
    for row in &rows {
        if !first_row {
            y_cursor += ROW_GAP;
        }
        first_row = false;
        let row_height = row
            .iter()
            .map(|&i| child_dims[i].1)
            .max()
            .unwrap_or(CARD_HEIGHT);
        row_origins.push((y_cursor, row_height));
        y_cursor += row_height;
    }

    type RowData = (usize, Vec<usize>, Vec<usize>, Vec<usize>);
    let mut row_data: Vec<RowData> = Vec::new();
    // (branch_y, child_centers, child_tops, local_indices)
    for (row_idx, row) in rows.iter().enumerate() {
        let (row_y, _row_h) = row_origins[row_idx];
        let row_width: usize = row.iter().map(|&i| child_dims[i].0).sum::<usize>()
            + H_GAP * row.len().saturating_sub(1);
        let mut x_cursor = origin_x + (block_w.saturating_sub(row_width)) / 2;

        let mut child_centers: Vec<usize> = Vec::new();
        let mut child_tops: Vec<usize> = Vec::new();

        for &local_idx in row {
            let child_id = child_ids[local_idx];
            let (cw_child, _) = child_dims[local_idx];
            place(
                child_id,
                x_cursor,
                row_y,
                available,
                card_widths,
                node_heights,
                children,
                dims,
                nodes,
                strokes,
            );
            let child_rect = nodes[child_id].unwrap();
            child_centers.push(child_rect.x + child_rect.w / 2);
            child_tops.push(child_rect.y);
            x_cursor += cw_child + H_GAP;
        }

        let branch_y = row_y.saturating_sub(2).max(parent_bottom_y + 1);
        row_data.push((branch_y, child_centers, child_tops, row.clone()));
    }

    // Choose a vertical lane for rows after the first to avoid passing through nodes.
    let min_y = parent_bottom_y + 1;
    let max_y = row_data.last().map(|r| r.0).unwrap_or(min_y);
    let lane_x = if rows.len() > 1 {
        find_lane(parent_center_x, min_y, max_y, nodes, available + MARGIN * 2)
    } else {
        parent_center_x
    };

    // Draw connectors per row.
    for (row_idx, (branch_y, child_centers, child_tops, _row)) in row_data.iter().enumerate() {
        if child_centers.is_empty() {
            continue;
        }

        let first_c = *child_centers.iter().min().unwrap();
        let last_c = *child_centers.iter().max().unwrap();

        // Horizontal branch for this row.
        for x in first_c..=last_c {
            strokes.push(Stroke {
                x,
                y: *branch_y,
                ch: '─',
                owner: id,
            });
        }

        if row_idx == 0 {
            // Direct trunk.
            for y in (parent_bottom_y + 1)..=*branch_y {
                strokes.push(Stroke {
                    x: parent_center_x,
                    y,
                    ch: '│',
                    owner: id,
                });
            }
            if parent_center_x >= first_c && parent_center_x <= last_c {
                let junction = if child_centers.contains(&parent_center_x) {
                    '┼'
                } else {
                    '┬'
                };
                strokes.push(Stroke {
                    x: parent_center_x,
                    y: *branch_y,
                    ch: junction,
                    owner: id,
                });
            } else {
                let (lo, hi) = if parent_center_x < first_c {
                    (parent_center_x, first_c)
                } else {
                    (last_c, parent_center_x)
                };
                for x in lo..=hi {
                    strokes.push(Stroke {
                        x,
                        y: *branch_y,
                        ch: if x == parent_center_x { '┬' } else { '─' },
                        owner: id,
                    });
                }
            }
        } else {
            // Detour via lane to avoid node interiors.
            let (lo, hi) = if lane_x < parent_center_x {
                (lane_x, parent_center_x)
            } else {
                (parent_center_x, lane_x)
            };
            for x in lo..=hi {
                strokes.push(Stroke {
                    x,
                    y: parent_bottom_y + 1,
                    ch: '─',
                    owner: id,
                });
            }
            for y in (parent_bottom_y + 1)..=*branch_y {
                strokes.push(Stroke {
                    x: lane_x,
                    y,
                    ch: '│',
                    owner: id,
                });
            }
            // Connect lane to branch range.
            let (blo, bhi) = if lane_x < first_c {
                (lane_x, first_c)
            } else if lane_x > last_c {
                (last_c, lane_x)
            } else {
                (lane_x, lane_x)
            };
            for x in blo..=bhi {
                strokes.push(Stroke {
                    x,
                    y: *branch_y,
                    ch: if x == lane_x {
                        if child_centers.contains(&lane_x) {
                            '┼'
                        } else {
                            '┬'
                        }
                    } else {
                        '─'
                    },
                    owner: id,
                });
            }
            if lane_x >= first_c && lane_x <= last_c {
                // ensure junction glyph at lane (already set above)
            }
        }

        // Child verticals and arrows.
        for (i, &center) in child_centers.iter().enumerate() {
            let child_top = child_tops[i];
            for y in (*branch_y + 1)..child_top {
                strokes.push(Stroke {
                    x: center,
                    y,
                    ch: '│',
                    owner: id,
                });
            }
            if *branch_y < child_top {
                strokes.push(Stroke {
                    x: center,
                    y: child_top.saturating_sub(1),
                    ch: '▼',
                    owner: id,
                });
            }
        }
    }

    let _ = (parent_center_x, parent_bottom_y);
}

fn find_lane(
    parent_center: usize,
    min_y: usize,
    max_y: usize,
    nodes: &[Option<NodeRect>],
    terminal_width: usize,
) -> usize {
    for radius in 0..=terminal_width {
        for candidate in [parent_center.saturating_sub(radius), parent_center + radius] {
            if candidate >= terminal_width {
                continue;
            }
            if !lane_intersects(candidate, min_y, max_y, nodes) {
                return candidate;
            }
        }
    }
    parent_center
}

fn lane_intersects(x: usize, min_y: usize, max_y: usize, nodes: &[Option<NodeRect>]) -> bool {
    for r in nodes.iter().filter_map(|r| *r) {
        if x < r.x || x >= r.x + r.w {
            continue;
        }
        let r_top = r.y;
        let r_bottom = r.y + r.h;
        if min_y < r_bottom && max_y >= r_top {
            return true;
        }
    }
    false
}

fn merge_strokes(strokes: Vec<Stroke>) -> Vec<Stroke> {
    let mut map: HashMap<(usize, usize), (char, usize)> = HashMap::new();
    for s in strokes {
        let entry = map.entry((s.x, s.y)).or_insert((s.ch, s.owner));
        entry.0 = merge_char(entry.0, s.ch);
        // Keep the first owner for the merged cell.
    }
    map.into_iter()
        .map(|((x, y), (ch, owner))| Stroke { x, y, ch, owner })
        .collect()
}

fn merge_char(existing: char, new: char) -> char {
    if existing == new {
        return existing;
    }
    if (existing == '─' && new == '│') || (existing == '│' && new == '─') {
        return '┼';
    }
    if new == '▼' || existing == '▼' {
        return '▼';
    }
    if new == '┼' || existing == '┼' {
        return '┼';
    }
    if (new == '┬' || new == '┴' || new == '├' || new == '┤')
        || (existing == '┬' || existing == '┴' || existing == '├' || existing == '┤')
    {
        // prefer junction over plain line
        if existing == '─' || existing == '│' {
            return new;
        }
        if new == '─' || new == '│' {
            return existing;
        }
        return '┼';
    }
    new
}
