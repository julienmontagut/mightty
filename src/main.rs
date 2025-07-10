use anyhow::Result;
use eframe::{egui::ViewportBuilder, NativeOptions};
use mightty::{config::Config, App};

#[tokio::main]
async fn main() -> Result<()> {
    let options: Config = Config::load();

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
