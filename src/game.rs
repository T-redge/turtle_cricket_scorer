use eframe::egui::{self, Color32, RichText};

use crate::{app::TeamRoles, innings::*, over::*, team::*};

pub struct Game {
    home_team: Team,
    away_team: Team,
    
    init_teams: bool,
    game_finished: bool,
    
    innings: Innings,
    over: Over,
}

impl Game {
    pub fn new() -> Self {
        Self {
            home_team: Team::new(),
            away_team: Team::new(),
            init_teams: false,
            
            game_finished: false,

            innings: Innings::new(5),
            over: Over::new(),
        }
    }
    fn return_init_team_complete(&self) -> bool {
        self.init_teams
    }
    pub fn return_game_finished(&self) -> bool {
        self.game_finished
    }

    pub fn play_match(&mut self, ctx: &egui::Context, home_team: (&str, &TeamRoles), away_team: (&str, &TeamRoles), toss_winner: (&str, &TeamRoles)) {
        if !self.return_init_team_complete() {
            set_team_info(self, home_team, away_team);
            set_player_lists(self, home_team.0, away_team.0);
        }
        create_info_panel(ctx, home_team, away_team, toss_winner);
        if !self.innings.return_innings_finished() {
            if !self.over.check_over_complete() {
                scorecard_panel(ctx, self);
            } else {
                
            }
        }
    }
}
fn create_info_panel(ctx: &egui::Context, home_team: (&str, &TeamRoles), away_team: (&str, &TeamRoles), toss_winner: (&str, &TeamRoles)) {
    egui::TopBottomPanel::top("Info").show(ctx, |ui| {
            ui.columns_const(|[teams, toss]| {
                teams.label(create_game_label(&("Teams: ".to_owned() + &home_team.0 + " - " + &away_team.0), 12.0));
                toss.label(create_game_label(
                    &("Toss won by ".to_owned() + toss_winner.0 + " and chose to " + (format!("{:?}",toss_winner.1)).as_str()), 12.0));
            });
        });
}
fn set_team_info(game: &mut Game, home_team: (&str, &TeamRoles), away_team: (&str, &TeamRoles)) {
    game.home_team.set_team_name(home_team.0);
    game.home_team.set_team_role(*home_team.1);
    game.away_team.set_team_name(away_team.0);
    game.away_team.set_team_role(*away_team.1);
}
fn set_player_lists(game: &mut Game, home_name: &str, away_name: &str) {
    game.home_team.set_team_vec(home_name);
    game.away_team.set_team_vec(away_name);
}
fn create_game_label(text: &str, size: f32) -> RichText {
    RichText::new(text).color(Color32::WHITE).monospace().size(size)
}
fn scorecard_panel(ctx: &egui::Context, game: &Game) {
    egui::SidePanel::left("id_scorecard")
        .resizable(false)
        .exact_width(600.0)
        .show(ctx, |ui| {
        let batting_team = match game.home_team.return_team_role() {
            TeamRoles::Bat => {game.home_team.return_team_name()}
            TeamRoles::Bowl => {game.away_team.return_team_name()}
            TeamRoles::Waiting => {String::new()}
        };

        let runs = game.innings.return_total_runs().to_string();
        let wickets = game.innings.return_total_wicket().to_string();
        let overs = game.innings.return_overs_completed().to_string();
        let balls = game.over.return_balls().to_string();
        ui.vertical_centered(|ui| {
            ui.label(create_game_label("Scoreboard", 30.0).underline());
        });
        ui.horizontal_wrapped(|ui| {
            ui.end_row();
            ui.label(create_game_label(&batting_team, 30.0));
            ui.label(create_game_label("\tscore", 12.0));
            ui.label(create_game_label(&(runs + "-" + &wickets), 30.0));
            ui.label(create_game_label("\tover", 12.0));
            ui.label(create_game_label(&(overs + "." + &balls), 30.0));
            ui.end_row();
            ui.label(create_game_label("Batters", 20.0).underline());
            ui.end_row();

        });
    });
}