//! Terminal buffer → ASCII text screenshot capture.
//!
//! Renders a ratatui `Buffer` to a plain text file with ANSI colors.
//! No image crate needed — pure text output for quick debugging.

use std::fs;
use std::path::Path;

/// Render a ratatui terminal buffer to an ASCII text file.
pub fn save_text(
    buf: &ratatui::buffer::Buffer,
    w: u16,
    h: u16,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = buffer_to_text(buf, w, h);
    fs::write(path, text)?;
    Ok(())
}

/// Render a ratatui buffer to a plain text string (no ANSI codes).
pub fn buffer_to_text(buf: &ratatui::buffer::Buffer, w: u16, h: u16) -> String {
    let mut lines = Vec::with_capacity(h as usize);

    for y in 0..h {
        let mut line = String::with_capacity(w as usize * 2);
        for x in 0..w {
            if let Some(cell) = buf.cell((x, y)) {
                let sym = cell.symbol();
                if sym.is_empty() {
                    line.push(' ');
                } else {
                    // ratatui uses multi-char for wide glyphs; take the first char
                    let ch = sym.chars().next().unwrap_or(' ');
                    line.push(ch);
                }
            } else {
                line.push(' ');
            }
        }
        // Trim trailing spaces
        let trimmed = line.trim_end();
        lines.push(trimmed.to_string());
    }

    // Remove trailing empty lines
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n") + "\n"
}

/// Render a ratatui buffer to an ANSI-colored text string.
#[allow(dead_code)]
pub fn buffer_to_ansi(buf: &ratatui::buffer::Buffer, w: u16, h: u16) -> String {
    use ratatui::style::Color;

    let mut lines = Vec::with_capacity(h as usize);

    for y in 0..h {
        let mut line = String::with_capacity(w as usize * 8);
        let mut prev_fg: Option<Color> = None;
        let mut prev_bg: Option<Color> = None;

        for x in 0..w {
            if let Some(cell) = buf.cell((x, y)) {
                let fg = cell.fg;
                let bg = cell.bg;

                // Only emit escape codes when color changes
                if Some(fg) != prev_fg || Some(bg) != prev_bg {
                    line.push_str(&ansi_color(fg, bg));
                    prev_fg = Some(fg);
                    prev_bg = Some(bg);
                }

                let sym = cell.symbol();
                let ch = sym.chars().next().unwrap_or(' ');
                line.push(ch);
            } else {
                line.push(' ');
            }
        }
        line.push_str("\x1b[0m"); // reset at end of line

        let trimmed_end = line.trim_end_matches("\x1b[0m").trim_end();
        if !trimmed_end.is_empty() {
            lines.push(format!("{}\x1b[0m", trimmed_end));
        } else {
            lines.push(String::new());
        }
    }

    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n") + "\n"
}

fn ansi_color(fg: ratatui::style::Color, bg: ratatui::style::Color) -> String {
    let fg_code = color_to_ansi_fg(fg);
    let bg_code = color_to_ansi_bg(bg);
    format!("\x1b[{};{}m", fg_code, bg_code)
}

fn color_to_ansi_fg(c: ratatui::style::Color) -> u8 {
    match c {
        ratatui::style::Color::Black => 30,
        ratatui::style::Color::Red => 31,
        ratatui::style::Color::Green => 32,
        ratatui::style::Color::Yellow => 33,
        ratatui::style::Color::Blue => 34,
        ratatui::style::Color::Magenta => 35,
        ratatui::style::Color::Cyan => 36,
        ratatui::style::Color::White => 37,
        ratatui::style::Color::DarkGray => 90,
        ratatui::style::Color::Gray => 37,
        ratatui::style::Color::Rgb(r, g, b) => {
            // Use 256-color approximation
            if r > 200 && g < 80 && b < 80 { 31 }       // red
            else if r < 80 && g > 180 && b < 80 { 32 }   // green
            else if r > 200 && g > 180 && b < 80 { 33 }  // yellow
            else if r < 80 && g < 80 && b > 200 { 34 }   // blue
            else if r > 200 && g < 80 && b > 200 { 35 }  // magenta
            else if r < 80 && g > 180 && b > 180 { 36 }  // cyan
            else if r > 180 && g > 180 && b > 180 { 37 } // white
            else if r > 100 && g > 100 && b > 100 { 90 } // dark gray
            else { 30 }                                    // black
        }
        _ => 37,
    }
}

fn color_to_ansi_bg(c: ratatui::style::Color) -> u8 {
    match c {
        ratatui::style::Color::Black => 40,
        ratatui::style::Color::Red => 41,
        ratatui::style::Color::Green => 42,
        ratatui::style::Color::Yellow => 43,
        ratatui::style::Color::Blue => 44,
        ratatui::style::Color::Magenta => 45,
        ratatui::style::Color::Cyan => 46,
        ratatui::style::Color::White => 47,
        ratatui::style::Color::DarkGray => 100,
        ratatui::style::Color::Gray => 47,
        ratatui::style::Color::Rgb(r, g, b) => {
            if r > 40 || g > 40 || b > 40 {
                // Non-default dark bg — map to dark bg
                if r > g && r > b { 41 }
                else if g > r && g > b { 42 }
                else if r > 180 && g > 180 { 43 }
                else { 40 }
            } else {
                40 // black bg
            }
        }
        _ => 40,
    }
}
