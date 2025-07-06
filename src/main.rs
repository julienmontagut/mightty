use app::App;
use config::Config;
use eframe::egui::ViewportBuilder;

mod app;
mod config;

#[tokio::main]
async fn main() -> eframe::Result {
    let options: Config = config::config();

    let viewport = ViewportBuilder::default()
        .with_title(options.window_title)
        .with_inner_size([options.width, options.height])
        .with_min_inner_size([400.0, 300.0]);
    let native_options = eframe::NativeOptions {
        viewport,
        vsync: options.enable_vsync,
        ..Default::default()
    };

    eframe::run_native(
        "Mightty",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
