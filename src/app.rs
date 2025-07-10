use crate::fonts;
use crate::term::{Terminal, TerminalConfig};
use eframe::egui::{CentralPanel, Color32, Context, Frame, Margin};
// use std::sync::{Arc, Mutex};

const MARGIN: Margin = Margin::same(8);

pub struct App {
    // state: Arc<Mutex<AppState>>,
    terminal: Terminal,
}

#[derive(Default)]
pub struct AppState;

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_fonts(fonts::custom_fonts());
        let config = TerminalConfig { font_size: 14.0 };
        Self {
            // state: Default::default(),
            terminal: Terminal::new(config),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.input(|input| {
            self.terminal.handle_input(input);
        });
        CentralPanel::default()
            .frame(Frame::default().fill(Color32::BLACK).inner_margin(MARGIN))
            .show(ctx, |ui| {
                self.terminal.show(ui);
            });
    }
}
