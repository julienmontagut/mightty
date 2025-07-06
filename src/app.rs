use eframe::egui;
use eframe::egui::ScrollArea;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct App {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle input events first to avoid holding the mutex during rendering
        self.handle_input(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(29, 31, 33)) // Background from config
                    .inner_margin(egui::Margin::same(8)),
            )
            .show(ctx, |ui| {
                // Lock mutex only for the duration needed to read state
                let (lines, cursor_row, cursor_col, rows, font_size) = {
                    let state = self.state.lock().unwrap();
                    (
                        state.lines.clone(),
                        state.cursor_row,
                        state.cursor_col,
                        state.rows,
                        state.font_size,
                    )
                };

                ScrollArea::vertical()
                    .max_height(rows as f32 * 20.0)
                    .show(ui, |ui| {
                        let painter = ui.painter();
                        for (i, line) in lines.iter().enumerate() {
                            let pos = egui::pos2(0.0, i as f32 * 20.0);
                            let color = if i == cursor_row {
                                egui::Color32::WHITE // Cursor line color
                            } else {
                                egui::Color32::from_rgb(200, 200, 200) // Default text color
                            };
                            painter.text(
                                pos,
                                egui::Align2::LEFT_TOP,
                                line,
                                egui::FontId::monospace(font_size as f32),
                                color,
                            );
                        }
                    });

                // Draw cursor
                if cursor_row < lines.len() {
                    let cursor_pos = egui::pos2(
                        cursor_col as f32 * (font_size as f32 * 0.6), // Approximate character width
                        cursor_row as f32 * 20.0,
                    );
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(cursor_pos, cursor_pos + egui::vec2(2.0, 20.0)),
                        0.0,
                        egui::Color32::WHITE, // Cursor color
                    );
                }
            });
    }
}

impl App {
    fn handle_input(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            for event in input.events.iter() {
                match event {
                    egui::Event::Text(text) => {
                        self.handle_text_input(text);
                    }
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers: _,
                        repeat: _,
                        physical_key: _,
                    } => {
                        self.handle_key_input(*key);
                    }
                    _ => {}
                }
            }
        });
    }

    fn handle_text_input(&mut self, text: &str) {
        if let Ok(mut state) = self.state.lock() {
            if state.cursor_row < state.lines.len() {
                for ch in text.chars() {
                    if ch.is_control() {
                        continue; // Skip control characters
                    }
                    let cursor_row = state.cursor_row;
                    let cursor_col = state.cursor_col;
                    if cursor_col <= state.lines[cursor_row].len() {
                        state.lines[cursor_row].insert(cursor_col, ch);
                        state.cursor_col += 1;
                    }
                }
            }
        }
    }

    fn handle_key_input(&mut self, key: egui::Key) {
        if let Ok(mut state) = self.state.lock() {
            match key {
                egui::Key::ArrowLeft => {
                    if state.cursor_col > 0 {
                        state.cursor_col -= 1;
                    }
                }
                egui::Key::ArrowRight => {
                    if state.cursor_row < state.lines.len() {
                        let line_len = state.lines[state.cursor_row].len();
                        if state.cursor_col < line_len {
                            state.cursor_col += 1;
                        }
                    }
                }
                egui::Key::ArrowUp => {
                    if state.cursor_row > 0 {
                        state.cursor_row -= 1;
                        // Adjust cursor column to fit within the new line
                        if state.cursor_row < state.lines.len() {
                            let line_len = state.lines[state.cursor_row].len();
                            if state.cursor_col > line_len {
                                state.cursor_col = line_len;
                            }
                        }
                    }
                }
                egui::Key::ArrowDown => {
                    if state.cursor_row + 1 < state.lines.len() {
                        state.cursor_row += 1;
                        // Adjust cursor column to fit within the new line
                        let line_len = state.lines[state.cursor_row].len();
                        if state.cursor_col > line_len {
                            state.cursor_col = line_len;
                        }
                    }
                }
                egui::Key::Backspace => {
                    if state.cursor_row < state.lines.len() {
                        let cursor_row = state.cursor_row;
                        let cursor_col = state.cursor_col;

                        if cursor_col > 0 && cursor_col <= state.lines[cursor_row].len() {
                            state.lines[cursor_row].remove(cursor_col - 1);
                            state.cursor_col -= 1;
                        } else if cursor_col == 0 && cursor_row > 0 {
                            // Join with previous line
                            let current_line = state.lines.remove(cursor_row);
                            let target_row = cursor_row - 1;
                            state.cursor_row = target_row;
                            let new_cursor_col = state.lines[target_row].len();
                            state.lines[target_row].push_str(&current_line);
                            state.cursor_col = new_cursor_col;
                        }
                    }
                }
                egui::Key::Delete => {
                    if state.cursor_row < state.lines.len() {
                        let cursor_row = state.cursor_row;
                        let cursor_col = state.cursor_col;
                        let line_len = state.lines[cursor_row].len();

                        if cursor_col < line_len {
                            state.lines[cursor_row].remove(cursor_col);
                        } else if cursor_col == line_len && cursor_row + 1 < state.lines.len() {
                            // Join with next line
                            let next_line = state.lines.remove(cursor_row + 1);
                            state.lines[cursor_row].push_str(&next_line);
                        }
                    }
                }
                egui::Key::Enter => {
                    if state.cursor_row < state.lines.len() {
                        let cursor_row = state.cursor_row;
                        let cursor_col = state.cursor_col;
                        let remaining = state.lines[cursor_row].split_off(cursor_col);
                        state.lines.insert(cursor_row + 1, remaining);
                        state.cursor_row += 1;
                        state.cursor_col = 0;
                    } else {
                        // Add new line at the end
                        state.lines.push(String::new());
                        state.cursor_row = state.lines.len() - 1;
                        state.cursor_col = 0;
                    }
                }
                egui::Key::Home => {
                    state.cursor_col = 0;
                }
                egui::Key::End => {
                    if state.cursor_row < state.lines.len() {
                        state.cursor_col = state.lines[state.cursor_row].len();
                    }
                }
                _ => {} // Ignore other keys
            }
        }
    }
}

struct AppState {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    cols: u16,
    rows: u16,
    // scroll_offset: usize,
    history: Vec<String>,
    font_size: u16,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            lines: vec![
                "Welcome to Mightty!".to_string(),
                "This is a simple terminal emulator.".to_string(),
                "You can type commands here.".to_string(),
                "Press Enter to execute a command.".to_string(),
                "Use the arrow keys to navigate.".to_string(),
            ],
            cursor_row: 0,
            cursor_col: 0,
            cols: 80, // Default terminal width
            rows: 24, // Default terminal height
            // scroll_offset: 0,
            history: vec![],
            font_size: 16, // Default font size
        }
    }
}
