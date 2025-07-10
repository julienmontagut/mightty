use eframe::egui::{
    text::LayoutJob, Color32, Event, FontId, InputState, Key, ScrollArea, TextFormat, Ui,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Terminal {
    config: TerminalConfig,
    state: Arc<Mutex<TerminalState>>,
}

impl Terminal {
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            config: config.clone(),
            ..Default::default()
        }
    }

    pub fn handle_input(&mut self, input: &InputState) {
        for event in input.events.iter() {
            match event {
                Event::Text(text) => {
                    self.handle_text_input(text);
                }
                Event::Key {
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

    fn handle_key_input(&mut self, key: Key) {
        if let Ok(mut state) = self.state.lock() {
            match key {
                Key::Backspace => {
                    if let Some(line) = state.lines.last_mut() {
                        if line.pop().is_none() && state.lines.len() > 1 {
                            state.lines.pop();
                        }
                    }
                }
                Key::Enter => {
                    state.lines.push(String::new());
                }
                _ => {} // Ignore other keys
            }
        }
    }

    pub fn show(&self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let mut job = LayoutJob::default();

            match self.state.lock() {
                Err(e) => {
                    job.append(
                        &format!("Error: {}", e),
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(self.config.font_size),
                            color: Color32::RED,
                            ..Default::default()
                        },
                    );
                }
                Ok(state) => {
                    job.append(
                        &state.lines.join("\n"),
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(self.config.font_size),
                            color: Color32::WHITE,
                            ..Default::default()
                        },
                    );

                    job.append("_", 8.0, TextFormat::default());
                }
            };

            ui.label(job)
        });
    }
}

#[derive(Clone)]
pub struct TerminalConfig {
    pub(crate) font_size: f32,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self { font_size: 14.0 }
    }
}

#[derive(Default)]
pub struct TerminalState {
    lines: Vec<String>,
}
