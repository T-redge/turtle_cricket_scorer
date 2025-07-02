use eframe::egui::{self, Color32, RichText};

use crate::{app::TeamRoles, innings::Innings, team::*};

pub struct Game {
    home_team: Team,
    away_team: Team,

    game_finished: bool,

    innings: Innings,
}

impl Game {
    pub fn new() -> Self {
        Self {
            home_team: Team::new(),
            away_team: Team::new(),

            game_finished: false,

            innings: Innings,
        }
    }
    pub fn return_game_finished(&self) -> bool {
        self.game_finished
    }

    pub fn play_match(&mut self, ctx: &egui::Context, home_team: (&str, &TeamRoles), away_team: (&str, &TeamRoles), toss_winner: (&str, &TeamRoles)) {
        self.home_team.set_team_name(home_team.0);
        self.away_team.set_team_name(away_team.0);
        egui::TopBottomPanel::top("Info").show(ctx, |ui| {
            ui.columns_const(|[teams, toss]| {
                teams.label(create_info_label(&("Teams: ".to_owned() + &self.home_team.return_team_name() + " - " + &self.away_team.return_team_name())));
                toss.label(create_info_label(&("Toss won by ".to_owned() + toss_winner.0 + " and chose to " + (format!("{:?}",toss_winner.1)).as_str())));
            });
        });
    }
}

fn create_info_label(text: &str) -> RichText {
    RichText::new(text).color(Color32::WHITE).monospace().size(12.0)
}