use anyhow::Result;
use app::App;
use config::Config;
use eframe::{egui::ViewportBuilder, NativeOptions};
use portable_pty::{CommandBuilder, PtySize};

mod app;
mod config;
mod fonts;
mod term;

#[tokio::main]
async fn main() -> Result<()> {
    let options: Config = config::config();

    let native_pty = portable_pty::native_pty_system();

    let pty = native_pty.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_height: 480,
        pixel_width: 640,
    })?;

    let command = CommandBuilder::new("bash");
    let _shell = pty.slave.spawn_command(command)?;

    let _reader = pty.master.try_clone_reader()?;

    writeln!(pty.master.take_writer()?, "ls -al")?;

    let viewport = ViewportBuilder::default()
        .with_title(options.window_title)
        .with_inner_size([options.width, options.height])
        .with_min_inner_size([400.0, 300.0]);

    let native_options = NativeOptions {
        viewport,
        vsync: options.enable_vsync,
        ..Default::default()
    };

    eframe::run_native(
        "Mightty",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;

    Ok(())
}
