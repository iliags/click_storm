//! The main entry point for the application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    env_logger::init();

    const WIDTH: f32 = 615.0;
    const HEIGHT: f32 = 300.0;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Set the window size
            .with_inner_size([750.0, HEIGHT])
            .with_min_inner_size([WIDTH, HEIGHT])
            // Allow resizing the window
            .with_resizable(true)
            // Start with focus on the window
            .with_active(true)
            // Disable the minimize button
            .with_minimize_button(false)
            // Set the window icon (optional)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Click Storm",
        native_options,
        Box::new(|cc| Ok(Box::new(click_storm::ClickStormApp::new(cc)))),
    )
}
