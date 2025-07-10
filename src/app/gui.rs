use crate::terminal::{Terminal, TerminalConfig};
use crate::ui::fonts;
use eframe::egui::{CentralPanel, Color32, Context, Frame, Margin};

const MARGIN: Margin = Margin::same(8);

pub struct App {
    terminal: Terminal,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_fonts(fonts::custom_fonts());
        let config = TerminalConfig { font_size: 14.0 };

        let mut terminal = Terminal::new(config);

        if let Err(error) = terminal.init() {
            panic!("Failed to initialize terminal: {}", error);
        }

        Self { terminal }
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
                if self.terminal.should_exit() {
                    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
                }
            });
    }
}
