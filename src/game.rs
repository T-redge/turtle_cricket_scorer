use eframe::egui::{self, Align2, Button, Color32, RichText};

use crate::{app::TeamRoles, innings::*, over::*, player::Player, player_selector::PlayerSelector, team::*};
#[derive(PartialEq)]
enum PositionSelect {
    Openers,
    Batter,
    Bowler,
}
pub struct Game {
    home_team: Team,
    away_team: Team,

    init_teams: bool,
    game_finished: bool,
    hide_player_list: bool,
    player_select: PlayerSelector,
    
    opening_bowler: bool,

    innings: Innings,
    over: Over,
    
    bat_1: Player,
    bat_2: Player,
    bat_1_sel: bool,
    bat_2_sel: bool,
    
    bowl_1: Player,
    bowl_2: Player,
    bowl_1_sel: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            home_team: Team::new(),
            away_team: Team::new(),

            init_teams: false,
            game_finished: false,

            hide_player_list: false,
            player_select: PlayerSelector::new(),
            opening_bowler: false,

            innings: Innings::new(5),
            over: Over::new(),
            bat_1: Player::new(),
            bat_2: Player::new(),
            bat_1_sel: false,
            bat_2_sel: false,
            
            bowl_1: Player::new(),
            bowl_2: Player::new(),
            bowl_1_sel: false,

        }
    }
    fn set_hide_player_list(&mut self, bool: bool) {
        self.hide_player_list = bool;
    }
    fn return_init_team_complete(&self) -> bool {
        self.init_teams
    }
    pub fn return_game_finished(&self) -> bool {
        self.game_finished
    }
    fn return_hide_player_list(&self) -> bool {
        self.hide_player_list
    }
    pub fn play_match(
        &mut self,
        ctx: &egui::Context,
        home_team: (&str, &TeamRoles),
        away_team: (&str, &TeamRoles),
        toss_winner: (&str, &TeamRoles),
    ) {
        if !self.return_init_team_complete() {
            set_team_info(self, home_team, away_team);
            set_player_lists(self, home_team.0, away_team.0);
        }
        create_info_panel(ctx, home_team, away_team, toss_winner);
        if !self.return_hide_player_list() {
            print_player_list(ctx, self);
        } else {
            let (bat_team,bowl_team) = match self.home_team.return_team_role() {
                TeamRoles::Bat => {
                    (self.home_team.return_team_name(), self.away_team.return_team_name())
                }
                TeamRoles::Bowl => {
                    (self.away_team.return_team_name(), self.home_team.return_team_name())
                }
                TeamRoles::Waiting => {(String::new(),String::new())}
            };
            
            if !self.innings.return_innings_finished() {
                if !self.bat_1_sel && !self.bat_2_sel {
                    create_player_selector(ctx, self, "Select opening batters!", &bat_team, PositionSelect::Openers);
                } else if !self.opening_bowler {
                    create_player_selector(ctx, self, "Select bowler!", &bowl_team, PositionSelect::Bowler);
                    if self.player_select.return_hide_player_selector() {
                        self.opening_bowler = true;
                    }
                } else {

                    if !self.over.check_over_complete() {
                        scorecard_panel(ctx, self);
                    } else {
                    }
                }
            }
        }
    }
}
fn create_info_panel(
    ctx: &egui::Context,
    home_team: (&str, &TeamRoles),
    away_team: (&str, &TeamRoles),
    toss_winner: (&str, &TeamRoles),
) {
    egui::TopBottomPanel::top("Info").show(ctx, |ui| {
        ui.columns_const(|[teams, toss]| {
            teams.label(create_game_label(
                &("Teams: ".to_owned() + &home_team.0 + " - " + &away_team.0),
                12.0,
            ));
            toss.label(create_game_label(
                &("Toss won by ".to_owned()
                    + toss_winner.0
                    + " and chose to "
                    + (format!("{:?}", toss_winner.1)).as_str()),
                12.0,
            ));
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
pub fn create_game_label(text: &str, size: f32) -> RichText {
    RichText::new(text)
        .color(Color32::WHITE)
        .monospace()
        .size(size)
}
fn scorecard_panel(ctx: &egui::Context, game: &Game) {
    egui::SidePanel::left("id_scorecard")
        .resizable(false)
        .exact_width(640.0)
        .show(ctx, |ui| {
            let batting_team = match game.home_team.return_team_role() {
                TeamRoles::Bat => game.home_team.return_team_name(),
                TeamRoles::Bowl => game.away_team.return_team_name(),
                TeamRoles::Waiting => String::new(),
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
                ui.columns_const(|[name,score]| {
                    name.label(create_game_label(&game.bat_1.return_profile().0, 16.0));
                    score.label(create_game_label(&game.bat_1.return_profile().1, 16.0));
                });
                ui.end_row();
                ui.columns_const(|[name,score]| {
                    name.label(create_game_label(&game.bat_2.return_profile().0, 16.0));
                    score.label(create_game_label(&game.bat_2.return_profile().1, 16.0));
                });
                ui.end_row();
                ui.label(create_game_label("Bowlers", 20.0).underline());
                ui.end_row();
                ui.columns_const(|[name,score]| {
                    name.label(create_game_label(&game.bowl_1.return_profile().0, 16.0));
                    score.label(create_game_label(&game.bowl_1.return_profile().1, 16.0));
                });
                ui.end_row();
                if !(game.bowl_2.return_profile().0 == "".to_string()) {

                    ui.columns_const(|[name,score]| {
                        name.label(create_game_label(&game.bowl_2.return_profile().0, 16.0));
                        score.label(create_game_label(&game.bowl_2.return_profile().1, 16.0));
                    });
                }
            });
        });
}
fn print_player_list(ctx: &egui::Context, game: &mut Game) {
    egui::TopBottomPanel::bottom("Button")
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui
                    .add_sized(
                        [150.0, 50.0],
                        Button::new(create_game_label("Confirm", 20.0)),
                    )
                    .clicked()
                {
                    game.set_hide_player_list(true);
                }
            });
        });
    egui::SidePanel::left("Home_team")
        .exact_width(640.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(create_game_label(&game.home_team.return_team_name(), 30.0).underline());
            ui.horizontal_wrapped(|ui| {
                let players = game.home_team.return_player_list();
                for x in players.iter() {
                    ui.label(create_game_label(x, 20.0));
                    ui.end_row();
                }
            });
        });
    egui::SidePanel::right("Away_team")
        .exact_width(640.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(create_game_label(&game.away_team.return_team_name(), 30.0).underline());
            ui.horizontal_wrapped(|ui| {
                let players = game.away_team.return_player_list();
                for x in players.iter() {
                    ui.label(create_game_label(x, 20.0));
                    ui.end_row();
                }
            });
        });
}
fn create_player_selector(ctx: &egui::Context, game: &mut Game, selection_text: &str, team_name: &str,pos_sel: PositionSelect) {
    egui::Window::new(selection_text)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            if pos_sel == PositionSelect::Openers {
                game.player_select.show_player_selector(ui,team_name);
                let player1 = game.player_select.return_chosen_player1();
                let player2 = game.player_select.return_chosen_player2();
                game.bat_1.set_name(&player1);
                if !game.bat_1_sel {
                    game.bat_1.set_name(&player1);
                    if game.player_select.return_hide_player_selector() {
                        game.bat_1_sel = true;
                        game.player_select.set_hide_player_select(false);
                    }
                }
                if !game.bat_2_sel {
                    game.bat_2.set_name(&player2);
                    if game.player_select.return_hide_player_selector() {
                        game.bat_2_sel = true;
                        game.player_select.set_hide_player_select(false);
                    }
                }
                
            }
            if pos_sel == PositionSelect::Bowler {
                game.player_select.show_player_selector(ui, team_name);
                let player = game.player_select.return_chosen_player1();
                if !game.bowl_1_sel {
                    game.bowl_1.set_name(&player);
                    game.bowl_1_sel = true;
                } else {
                    game.bowl_2.set_name(&player);
                    game.bowl_1_sel = false;
                }
            }
            if pos_sel == PositionSelect::Batter {
                game.player_select.show_player_selector(ui, team_name);
                let player = game.player_select.return_chosen_player1();
                if !game.bat_1_sel {
                    game.bat_1.set_name(&player);
                    game.bat_1_sel = true;
                }
                if !game.bat_2_sel {
                    game.bat_2.set_name(&player);
                    game.bat_2_sel = true;
                }
            }
        });
}
