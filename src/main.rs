use eframe::{NativeOptions, egui::ViewportBuilder, icon_data};

mod app;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_icon(
                icon_data::from_png_bytes(
                    &include_bytes!("../assets/favicon.png")[..],
                )
                .expect("Failed to load icon"),
            )
            .with_min_inner_size([800., 600.])
            .with_inner_size([1280., 720.]),
        ..Default::default()
    };
    eframe::run_native(
        "RESS",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
