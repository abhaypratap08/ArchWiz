use std::env;

pub fn terminal_width(override_width: Option<usize>) -> usize {
    if let Some(w) = override_width {
        return w.max(20);
    }
    if let Ok(cols) = env::var("COLUMNS")
        && let Ok(n) = cols.parse::<usize>()
        && n >= 20
    {
        return n;
    }
    if let Some((w, _)) = terminal_size::terminal_size() {
        let w = w.0 as usize;
        if w >= 20 {
            return w;
        }
    }
    100
}

pub fn use_color(no_color: bool) -> bool {
    if no_color {
        return false;
    }
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    // stdout terminal check
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}
