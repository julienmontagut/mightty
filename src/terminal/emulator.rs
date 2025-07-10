use crate::parser::{TerminalEvent, VteParser};
use crate::pty::Pty;
use crate::terminal::state::TerminalState;
use anyhow::Result;
use eframe::egui::{
    text::LayoutJob, Color32, Event, FontId, InputState, Key, ScrollArea, TextFormat, Ui,
};
use portable_pty::PtySize;
use std::sync::{Arc, Mutex};

pub struct Terminal {
    config: TerminalConfig,
    state: Arc<Mutex<TerminalState>>,
    pty: Option<Pty>,
    vte_parser: VteParser,
}

impl Terminal {
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            config: config.clone(),
            state: Arc::new(Mutex::new(TerminalState::with_size(24, 80))),
            pty: None,
            vte_parser: VteParser::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        let size = PtySize {
            rows: 24,
            cols: 80,
            pixel_height: 480,
            pixel_width: 640,
        };

        self.pty = Some(Pty::new(size)?);

        Ok(())
    }

    fn pty(&self) -> &Pty {
        self.pty.as_ref().expect("Missing PTY manager")
    }

    fn pty_mut(&mut self) -> &mut Pty {
        self.pty.as_mut().expect("Missing PTY manager")
    }

    pub fn handle_input(&mut self, input: &InputState) {
        // Only handle input if PTY is alive
        if !self.pty_mut().is_alive() {
            return;
        }

        for event in input.events.iter() {
            match event {
                Event::Text(text) => {
                    if let Err(e) = self.handle_text_input(text) {
                        eprintln!("Failed to handle text input: {}", e);
                    }
                }
                Event::Key {
                    key,
                    pressed: true,
                    modifiers: _,
                    repeat: _,
                    physical_key: _,
                } => {
                    if let Err(e) = self.handle_key_input(*key) {
                        eprintln!("Failed to handle key input: {}", e);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_text_input(&mut self, text: &str) -> Result<()> {
        self.pty().write_input(text)
    }

    fn handle_key_input(&mut self, key: Key) -> Result<()> {
        let input = match key {
            Key::Backspace => "\x08",
            Key::Enter => "\r",
            Key::Tab => "\t",
            Key::Escape => "\x1b",
            Key::ArrowUp => "\x1b[A",
            Key::ArrowDown => "\x1b[B",
            Key::ArrowRight => "\x1b[C",
            Key::ArrowLeft => "\x1b[D",
            Key::Home => "\x1b[H",
            Key::End => "\x1b[F",
            Key::PageUp => "\x1b[5~",
            Key::PageDown => "\x1b[6~",
            Key::Delete => "\x1b[3~",
            _ => return Ok(()),
        };

        self.pty().write_input(input)
    }

    fn process_pty_output(&mut self, data: &[u8]) {
        let events = self.vte_parser.parse(data);

        if let Ok(mut state) = self.state.lock() {
            for event in events {
                match event {
                    TerminalEvent::Print(c) => {
                        state.print_char(c);
                    }
                    TerminalEvent::Execute(b) => {
                        state.execute_control(b);
                    }
                    TerminalEvent::CsiDispatch(params, intermediates, ignore, c) => {
                        state.handle_csi(params, &intermediates, ignore, c);
                    }
                    _ => {
                        // Handle other VTE events as needed
                    }
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        while let Some(data) = self.pty_mut().try_read_output() {
            self.process_pty_output(&data);
        }

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
                        &state.get_display_text(),
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(self.config.font_size),
                            color: Color32::WHITE,
                            ..Default::default()
                        },
                    );

                    if state.show_cursor {
                        job.append(
                            "_",
                            8.0,
                            TextFormat {
                                font_id: FontId::monospace(self.config.font_size),
                                color: Color32::GREEN,
                                ..Default::default()
                            },
                        );
                    }
                }
            };

            ui.label(job)
        });
    }

    pub fn should_exit(&mut self) -> bool {
        !self.pty_mut().is_alive()
    }

    pub fn resize(&mut self, size: PtySize) {
        if let Err(e) = self.pty().resize(size) {
            eprintln!("Failed to resize PTY: {}", e);
        }
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
