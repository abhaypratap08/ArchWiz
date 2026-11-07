mod canvas;
mod terminal;

pub use canvas::Canvas;
pub use terminal::{terminal_width, use_color};

use crate::graph::GraphNode;
use crate::layout::{Layout, NodeRect};
use crate::theme;
use unicode_width::UnicodeWidthChar;

pub fn render(graph: &crate::graph::Graph, layout: &Layout, use_color: bool) {
    if graph.nodes.is_empty() {
        println!("[ EMPTY ] No graph nodes to render.");
        return;
    }

    let width = layout.width.max(20);
    let height = layout.height.max(1);

    let mut canvas = Canvas::new(width, height);

    let conn_color = theme::connection_color();
    for stroke in &layout.strokes {
        if stroke.x >= width || stroke.y >= height {
            continue;
        }
        let existing = canvas.cells[stroke.y][stroke.x].ch;
        let allow = match (existing, stroke.ch) {
            (_, ' ') => true,
            (' ', _) => true,
            ('─' | '│', '┬' | '┼' | '▼' | '├' | '┴') => true,
            ('┬' | '┼' | '▼', '─' | '│') => false,
            _ => true,
        };
        if allow {
            canvas.put(stroke.x, stroke.y, stroke.ch, conn_color);
        }
    }

    let preview_limit = layout.preview_limit;
    for node in &graph.nodes {
        if let Some(rect) = layout.nodes[node.id] {
            draw_card(&mut canvas, rect, node, preview_limit);
        }
    }

    println!();
    canvas.print(use_color);
    println!();
}

fn draw_card(canvas: &mut Canvas, rect: NodeRect, node: &GraphNode, preview_limit: usize) {
    let color = theme::kind_color(&node.kind);
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;

    // Clear interior to avoid connector bleed.
    for yy in y..y + h {
        for xx in x + 1..x + w.saturating_sub(1) {
            canvas.put(xx, yy, ' ', 0);
        }
    }

    canvas.put(x, y, '╭', color);
    for i in 1..w.saturating_sub(1) {
        canvas.put(x + i, y, '─', color);
    }
    if w > 1 {
        canvas.put(x + w - 1, y, '╮', color);
    }

    draw_content(canvas, x, y + 1, w, &node.label, color, color);
    draw_content(
        canvas,
        x,
        y + 2,
        w,
        &node.subtitle,
        color,
        theme::metadata_color(),
    );

    let shown = node.details.len().min(preview_limit);
    let truncated = node.details.len() > preview_limit;
    for (idx, detail) in node.details.iter().take(shown).enumerate() {
        let dc = node
            .detail_colors
            .get(idx)
            .copied()
            .unwrap_or(theme::metadata_color());
        draw_content(canvas, x, y + 3 + idx, w, detail, color, dc);
    }
    if truncated {
        let remaining = node.details.len() - shown;
        draw_content(
            canvas,
            x,
            y + 3 + shown,
            w,
            &format!("+{} MORE · ENTER TO INSPECT", remaining),
            color,
            theme::metadata_color(),
        );
    }

    let bottom = y + h - 1;
    canvas.put(x, bottom, '╰', color);
    for i in 1..w.saturating_sub(1) {
        canvas.put(x + i, bottom, '─', color);
    }
    if w > 1 {
        canvas.put(x + w - 1, bottom, '╯', color);
    }

    // Side borders for interior rows.
    for yy in y + 1..bottom {
        canvas.put(x, yy, '│', color);
        if w > 1 {
            canvas.put(x + w - 1, yy, '│', color);
        }
    }
}

fn draw_content(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    text: &str,
    border_color: u8,
    text_color: u8,
) {
    canvas.put(x, y, '│', border_color);
    if width > 1 {
        canvas.put(x + 1, y, ' ', 0);
    }
    let available = width.saturating_sub(4);
    let content = truncate_display(text, available);

    let mut cx = x + 2;
    for ch in content.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1).max(1);
        canvas.put(cx, y, ch, text_color);
        if w > 1 {
            for k in 1..w {
                canvas.put(cx + k, y, '\0', 0);
            }
        }
        cx += w;
    }

    // pad
    if width >= 2 {
        canvas.put(x + width.saturating_sub(2), y, ' ', 0);
        canvas.put(x + width.saturating_sub(1), y, '│', border_color);
    }
}

fn truncate_display(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    let mut result = String::new();
    let mut width = 0usize;
    for ch in text.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1);
        if width + w > max_width {
            if width < max_width {
                result.push('…');
            }
            break;
        }
        result.push(ch);
        width += w;
    }
    result
}
