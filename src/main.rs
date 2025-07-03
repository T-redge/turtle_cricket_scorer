use eframe::egui::{Vec2, ViewportBuilder};
pub mod app;
pub mod game;
pub mod innings;
pub mod over;
pub mod player;
pub mod player_selector;
pub mod team;
pub mod team_selector;
use crate::app::*;
fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2 {
                x: 1280.0,
                y: 720.0,
            }),
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
