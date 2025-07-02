use eframe::egui::{self, Color32, Label, RichText};
use crate::game::*;
pub struct Scoreboard {
    game: Game,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
           game: Game::new(),
        }
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let teams = self.game.return_team_names();
            ui.add(Label::new(create_team_labels(teams)));
        });
    }
}

fn create_team_labels(teams: (String,String)) -> RichText {
    let mut teams_label = String::from("Teams: ");
    teams_label.push_str(&teams.0);
    teams_label.push_str(" - ");
    teams_label.push_str(&teams.1);
    RichText::new(teams_label).color(Color32::WHITE).monospace().size(20.0)
}