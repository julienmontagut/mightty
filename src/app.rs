use eframe::egui::text::LayoutJob;
use eframe::egui::{FontFamily, FontId, Margin, ScrollArea, TextFormat};
use eframe::{egui, CreationContext};
use std::sync::{Arc, Mutex};

const MARGIN: Margin = Margin::same(8);

pub struct App {
    state: Arc<Mutex<AppState>>,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            state: Default::default(),
        }
    }

    pub fn lines(&self) -> Vec<String> {
        let state = self.state.lock().unwrap();
        state.lines.clone()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_input(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::BLACK)
                    .inner_margin(MARGIN),
            )
            .show(ctx, |ui| {
                let font_size = {
                    let state = self.state.lock().unwrap();
                    state.font_size
                };

                ScrollArea::vertical().show(ui, |ui| {
                    let mut job = LayoutJob::default();

                    job.append(
                        &self.lines().join("\n"),
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(font_size),
                            color: egui::Color32::WHITE,
                            ..Default::default()
                        },
                    );

                    job.append("_", 8.0, TextFormat::default());

                    ui.label(job);
                });
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
            for ch in text.chars() {
                if ch.is_control() {
                    continue;
                }
                if state.lines.is_empty() {
                    state.lines.push(String::new());
                }
                let len = state.lines.len();
                state.lines[len - 1].push(ch);
            }
        }
    }

    fn handle_key_input(&mut self, key: egui::Key) {
        if let Ok(mut state) = self.state.lock() {
            match key {
                egui::Key::Backspace => {
                    if let Some(line) = state.lines.last_mut() {
                        if let None = line.pop() {
                            if state.lines.len() > 1 {
                                state.lines.pop();
                            }
                        }
                    }
                }
                egui::Key::Enter => {
                    state.lines.push(String::new());
                }
                _ => {} // Ignore other keys
            }
        }
    }
}

struct AppState {
    lines: Vec<String>,
    cols: u16,
    rows: u16,
    // scroll_offset: usize,
    history: Vec<String>,
    font_size: f32,
    font_name: Option<String>,
}

impl AppState {
    pub fn lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn font_id(&self) -> FontId {
        let font_name = &self.font_name;
        if let Some(_name) = font_name {
            let family = FontFamily::Name(Arc::from("monospace"));
            FontId::new(self.font_size, family)
        } else {
            FontId::monospace(self.font_size)
        }
    }
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
            cols: 80,
            rows: 24,
            // scroll_offset: 0,
            history: vec![],
            font_size: 16.0,
            font_name: None,
        }
    }
}
