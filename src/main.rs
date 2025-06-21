use eframe::egui::{Vec2, ViewportBuilder};
mod buttons;
mod innings;
mod player;
mod scoreboard;
mod team;
use crate::scoreboard::*;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2 { x: 640.0, y: 360.0 }),
            ..Default::default()
        },
        ..Default::default()    
    };
    let home_team = "Edgewater";
    let away_team = "Kingsway";
    eframe::run_native(
        "Scorer",
        native_options,
        Box::new(|cc| Ok(Box::new(Scoreboard::new(cc, home_team, away_team)))),
    )
    .unwrap();
}
