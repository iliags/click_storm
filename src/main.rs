//! The main entry point for the application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    env_logger::init();

    // TODO: Add an options file that is read in at startup
    // TODO: If the file doesn't exist, create it with default values
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Set the window size
            .with_inner_size([472.0, 295.0])
            .with_min_inner_size([472.0, 295.0])
            // Always on top for easy access
            .with_always_on_top()
            // No resizing
            .with_resizable(false)
            // Start with focus on the window
            .with_active(true)
            // Set the window icon
            .with_icon(
                // NOTE: Adding an icon is optional
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
