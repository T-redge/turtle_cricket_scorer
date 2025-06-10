use eframe::egui::{Vec2, ViewportBuilder};
pub mod app;
pub mod extras;
pub mod functions;
pub mod innings;
pub mod over;
pub mod player;
pub mod team;
use app::*;
fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2 { x: 640.0, y: 360.0 }),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Scorer",
        native_options,
        Box::new(|cc| Ok(Box::new(Scoreboard::new(cc)))),
    )
    .unwrap();
}
