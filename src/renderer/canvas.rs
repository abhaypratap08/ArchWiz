#[allow(dead_code)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub color: u8,
}

impl Cell {
    pub fn blank() -> Self {
        Self { ch: ' ', color: 0 }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![Cell::blank(); width]; height],
        }
    }

    pub fn put(&mut self, x: usize, y: usize, ch: char, color: u8) {
        if y < self.height
            && let Some(row) = self.cells.get_mut(y)
            && x < row.len()
        {
            row[x].ch = ch;
            row[x].color = color;
        }
    }

    pub fn print(&self, use_color: bool) {
        for row in &self.cells {
            let end = row
                .iter()
                .rposition(|c| c.ch != ' ' && c.ch != '\0')
                .map_or(0, |i| i + 1);
            let mut current_color = 0u8;
            for cell in &row[..end] {
                if cell.ch == '\0' {
                    continue;
                }
                if use_color && current_color != cell.color {
                    if cell.color == 0 {
                        print!("\x1b[0m");
                    } else {
                        print!("\x1b[38;5;{}m", cell.color);
                    }
                    current_color = cell.color;
                }
                print!("{}", cell.ch);
            }
            if use_color && current_color != 0 {
                print!("\x1b[0m");
            }
            println!();
        }
    }
}
