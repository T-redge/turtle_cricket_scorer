use crate::{
    buttons::*,
    innings::*,
    player::{BattingStatus, BowlingStatus, bat::BatterStrike},
};
use eframe::egui::{self, Align2, Button, Color32, Label, RichText};
pub struct Scoreboard {
    pub innings: Innings,
    innings_started: bool,
    extra_button_hidden: bool,
    batters_picked: bool,
    on_strike_selected: bool,
    bowler_picked: bool,

    player_number: Vec<usize>,
    selected_batters: Vec<bool>,
    selected_bowlers: Vec<bool>,
}

impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.innings.match_ball_event();
        if !self.innings_started {
            team_lists(ctx, self);
        } else if !self.innings.check_innings_finished() {
            if !self.batters_picked || self.innings.check_wicket_taken() {
                pick_batters(ctx, self);
            } else if !self.on_strike_selected {
                set_batter_strike(ctx, self);
            } else if !self.bowler_picked {
                pick_bowler(ctx, self);
            } else {
                if self.innings.check_over_finished() {
                    over_recap(ctx, self);
                }
                team_scores(ctx, self);
                batter_scores(ctx, self);
                bowler_scores(ctx, self);
                extra_scores(ctx, self);
                button_bar(ctx, self);
            }
        } else {
            innings_recap(ctx, self);
        }
    }
}

impl Scoreboard {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        home_team: &'static str,
        away_team: &'static str,
    ) -> Self {
        Self {
            innings: Innings::new(home_team, away_team),
            innings_started: false,
            extra_button_hidden: false,
            batters_picked: false,
            on_strike_selected: false,
            bowler_picked: false,

            player_number: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            selected_batters: vec![false; 11],
            selected_bowlers: vec![false; 11],
        }
    }
    pub fn set_hide_button_bool(&mut self, set: bool) {
        self.extra_button_hidden = set;
    }
    pub fn return_hide_button_bool(&self) -> bool {
        self.extra_button_hidden
    }
    pub fn set_bowler_picked(&mut self, set: bool) {
        self.bowler_picked = set;
    }
}
fn team_lists(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (home_team, away_team) = scoreboard.innings.return_team_names();
        let (home_team_players, away_team_players) = scoreboard.innings.return_team_lists();
        ui.columns_const(|[home, away]| {
            home.add(Label::new(
                RichText::new(home_team)
                    .underline()
                    .size(24.0)
                    .monospace()
                    .color(Color32::WHITE),
            ));
            home.add(Label::new(
                RichText::new(home_team_players)
                    .monospace()
                    .color(Color32::WHITE),
            ));
            away.add(Label::new(
                RichText::new(away_team)
                    .underline()
                    .size(24.0)
                    .monospace()
                    .color(Color32::WHITE),
            ));
            away.add(Label::new(
                RichText::new(away_team_players)
                    .monospace()
                    .color(Color32::WHITE),
            ));
        });
        ui.vertical_centered_justified(|ui| {
            if ui
                .add_sized([150.0, 50.0], Button::new("Start inning"))
                .clicked()
            {
                scoreboard.innings_started = true;
            }
        });
    });
}
fn team_scores(ctx: &egui::Context, scoreboard: &Scoreboard) {
    egui::TopBottomPanel::top("id_Team_Scores")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.columns_const(|[teams, scores, overs]| {
                    teams.add(Label::new(
                        RichText::new(scoreboard.innings.return_team_names().0)
                            .monospace()
                            .color(Color32::WHITE)
                            .size(20.0),
                    ));
                    teams.end_row();
                    teams.add(Label::new(
                        RichText::new(scoreboard.innings.return_team_names().1)
                            .monospace()
                            .color(Color32::WHITE)
                            .size(20.0),
                    ));
                    scores.add(Label::new(
                        RichText::new(String::from(scoreboard.innings.return_team_score()))
                            .monospace()
                            .color(Color32::WHITE)
                            .size(20.0),
                    ));
                    scores.add(Label::new(
                        RichText::new(String::from("0/0"))
                            .monospace()
                            .color(Color32::WHITE)
                            .size(20.0),
                    ));
                    overs.add(Label::new(
                        RichText::new(scoreboard.innings.return_overs_label())
                            .monospace()
                            .color(Color32::WHITE)
                            .size(19.0),
                    ));
                });
            });
        });
}
fn batter_scores(ctx: &egui::Context, scoreboard: &Scoreboard) {
    egui::TopBottomPanel::top("id_Batter_Scores")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.add(Label::new(
                RichText::new("\nBatters")
                    .color(Color32::WHITE)
                    .monospace()
                    .size(20.0)
                    .underline(),
            ));
            let batters = scoreboard.innings.batting_team.return_batting_pair();
            let b_1 = scoreboard
                .innings
                .batting_team
                .return_player_batter_score(batters.0);
            let b_2 = scoreboard
                .innings
                .batting_team
                .return_player_batter_score(batters.1);
            ui.columns_const(|[ui_1, ui_2]| {
                ui_1.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(b_1.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    if scoreboard.innings.batting_team.players[batters.0].return_batter_strike()
                        == BatterStrike::OnStrike
                    {
                        ui.add(Label::new(
                            RichText::new("*")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(b_2.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    if scoreboard.innings.batting_team.players[batters.1].return_batter_strike()
                        == BatterStrike::OnStrike
                    {
                        ui.add(Label::new(
                            RichText::new("*")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                });
                ui_2.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(b_1.1)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(b_2.1)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                });
            });
        });
}
fn bowler_scores(ctx: &egui::Context, scoreboard: &Scoreboard) {
    egui::TopBottomPanel::top("id_Bowler_Stats")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.add(Label::new(
                RichText::new("\nBowlers")
                    .color(Color32::WHITE)
                    .monospace()
                    .size(20.0)
                    .underline(),
            ));
            let p_1 = scoreboard.innings.bowling_team.return_current_bowler();
            let p_2 = scoreboard.innings.bowling_team.return_last_over_bowler();

            ui.columns_const(|[ui_1, ui_2]| {
                ui_1.horizontal_wrapped(|ui| {
                    if p_1 < 11 {
                        let bowler = scoreboard
                            .innings
                            .bowling_team
                            .return_player_bowler_score(p_1);
                        ui.add(Label::new(
                            RichText::new(bowler.0)
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    } else {
                        ui.add(Label::new(
                            RichText::new(" ")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                    ui.end_row();
                    if p_2 < 11 {
                        let past_bowler = scoreboard
                            .innings
                            .bowling_team
                            .return_player_bowler_score(p_2);
                        ui.add(Label::new(
                            RichText::new(past_bowler.0)
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    } else {
                        ui.add(Label::new(
                            RichText::new("")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                });
                ui_2.horizontal_wrapped(|ui| {
                    if p_1 < 11 {
                        let bowler = scoreboard
                            .innings
                            .bowling_team
                            .return_player_bowler_score(p_1);
                        ui.add(Label::new(
                            RichText::new(bowler.1)
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    } else {
                        ui.add(Label::new(
                            RichText::new("")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                    ui.end_row();
                    if p_2 < 11 {
                        let past_bowler = scoreboard
                            .innings
                            .bowling_team
                            .return_player_bowler_score(p_2);
                        ui.add(Label::new(
                            RichText::new(past_bowler.1)
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    } else {
                        ui.add(Label::new(
                            RichText::new("")
                                .color(Color32::WHITE)
                                .monospace()
                                .size(12.0),
                        ));
                    }
                });
            });
        });
}
fn extra_scores(ctx: &egui::Context, scoreboard: &Scoreboard) {
    egui::TopBottomPanel::top("id_Extra_Scores")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(scoreboard.innings.return_team_extras())
                        .color(Color32::WHITE)
                        .monospace()
                        .size(16.0),
                ));
            });
        });
}
fn button_bar(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::TopBottomPanel::top("id_Button_bar")
        .exact_height(100.0)
        .show(ctx, |ui| {
            if !scoreboard.innings.check_over_finished() {
                ui.add_enabled_ui(true, |ui| {
                    ui.columns_const(|[dot, run, extra, wicket]| {
                        dot_ball_button(dot, scoreboard);
                        runs_ball_button(run, scoreboard);
                        if !scoreboard.return_hide_button_bool() {
                            extra_ball_button(extra, scoreboard);
                        } else {
                            extra.columns_const(|[col_1, col_2]| {
                                wide_ball_button(col_1, scoreboard);
                                noball_ball_button(col_2, scoreboard);
                                bye_ball_button(col_1, scoreboard);
                                legbye_ball_button(col_2, scoreboard);
                            });
                        }
                        wicket_ball_button(wicket, scoreboard);
                    });
                });
            }
        });
}
fn innings_recap(ctx: &egui::Context, scoreboard: &Scoreboard) {
    egui::CentralPanel::default().show(ctx, |ui| {
        innings_scorecard(ui, scoreboard);
    });
}
fn over_recap(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Over recap")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(Label::new(RichText::new("End of over!")));
            ui.add(Label::new(
                RichText::new(scoreboard.innings.return_over_totals())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(20.0),
            ));
            new_over_button(ui, scoreboard);
        });
}
fn pick_batters(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Select Batter")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            for p in &mut scoreboard.player_number {
                if scoreboard.innings.batting_team.players[*p].return_batting_status()
                    != BattingStatus::Waiting
                {
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(
                            &mut scoreboard.selected_batters[*p],
                            scoreboard.innings.batting_team.players[*p].return_name(),
                        ),
                    );
                } else {
                    ui.add_enabled(
                        true,
                        egui::Checkbox::new(
                            &mut scoreboard.selected_batters[*p],
                            scoreboard.innings.batting_team.players[*p].return_name(),
                        ),
                    );
                }
            }
            if ui
                .add_sized(
                    egui::Vec2 { x: 150.0, y: 50.0 },
                    Button::new("Select Batters"),
                )
                .clicked()
            {
                for player in &scoreboard.player_number {
                    if scoreboard.selected_batters[*player] {
                        scoreboard.innings.batting_team.players[*player]
                            .set_batter_status(crate::player::BattingStatus::Batting);
                        if scoreboard.innings.check_wicket_taken() {
                            scoreboard.innings.batting_team.players[*player]
                                .set_batter_strike(BatterStrike::OnStrike);
                        }
                    }
                }
                scoreboard.batters_picked = true;
                for player in &scoreboard.player_number {
                    scoreboard.selected_batters[*player] = false;
                }
                scoreboard.innings.set_wicket_taken(false);
            }
        });
}
fn set_batter_strike(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Choose batter on-strike")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            let batters = scoreboard.innings.batting_team.return_batting_pair();
            ui.columns_const(|[bat_1, bat_2]| {
                if bat_1
                    .add_sized(
                        [150.0, 50.0],
                        Button::new(
                            RichText::new(
                                scoreboard.innings.batting_team.players[batters.0].return_name(),
                            )
                            .color(Color32::WHITE)
                            .monospace()
                            .size(20.0),
                        ),
                    )
                    .clicked()
                {
                    scoreboard.innings.batting_team.players[batters.0]
                        .set_batter_strike(BatterStrike::OnStrike);
                    scoreboard.on_strike_selected = true;
                }
                if bat_2
                    .add_sized(
                        [150.0, 50.0],
                        Button::new(
                            RichText::new(
                                scoreboard.innings.batting_team.players[batters.1].return_name(),
                            )
                            .color(Color32::WHITE)
                            .monospace()
                            .size(20.0),
                        ),
                    )
                    .clicked()
                {
                    scoreboard.innings.batting_team.players[batters.1]
                        .set_batter_strike(BatterStrike::OnStrike);
                    scoreboard.on_strike_selected = true;
                }
            });
        });
}
fn pick_bowler(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Select Bowler")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            for p in &scoreboard.player_number {
                if scoreboard.innings.bowling_team.players[*p].return_bowler_status()
                    == BowlingStatus::BowledLastOver
                {
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(
                            &mut scoreboard.selected_bowlers[*p],
                            scoreboard.innings.bowling_team.players[*p].return_name(),
                        ),
                    );
                } else {
                    ui.add_enabled(
                        true,
                        egui::Checkbox::new(
                            &mut scoreboard.selected_bowlers[*p],
                            scoreboard.innings.bowling_team.players[*p].return_name(),
                        ),
                    );
                }
            }

            if ui
                .add_sized(
                    egui::Vec2 { x: 150.0, y: 50.0 },
                    Button::new("Select Bowler"),
                )
                .clicked()
            {
                for player in &scoreboard.player_number {
                    if scoreboard.selected_bowlers[*player] {
                        scoreboard.innings.bowling_team.players[*player]
                            .set_bowler_status(crate::player::BowlingStatus::Bowling);
                        scoreboard.innings.bowling_team.players[*player].set_bowler_bool(true);
                    }
                }
                scoreboard.bowler_picked = true;
                for player in &scoreboard.player_number {
                    scoreboard.selected_bowlers[*player] = false;
                }
            }
        });
}
fn innings_scorecard(ui: &mut egui::Ui, scoreboard: &Scoreboard) {
    ui.horizontal(|ui| {
        ui.add(Label::new(
            RichText::new(scoreboard.innings.batting_team.return_team_name() + " Innings")
                .color(Color32::WHITE)
                .monospace()
                .underline()
                .size(16.0),
        ));
    });
    ui.horizontal(|ui| {
        batting_scorecard(ui, scoreboard);
    });
    ui.horizontal(|ui| {
        extras_scoreboard(ui, scoreboard);
    });
    ui.horizontal(|ui| {
        ui.add(Label::new(
        RichText::new("Bowling Figures")
            .color(Color32::WHITE)
            .monospace()
            .size(12.0)
            .underline(),
        ));
    });
    ui.horizontal(|ui| {
        bowling_scorecard(ui, scoreboard);
    });
}
fn batting_scorecard(ui: &mut egui::Ui, scoreboard: &Scoreboard) {
    ui.columns_const(|[name, dismissal, bowler, bat_score]| {
        for player in 0..=10 {
            let player_score =
                scoreboard.innings.batting_team.players[player].return_scoreboard_profile();
            name.add(Label::new(
                RichText::new(player_score.0)
                    .color(Color32::WHITE)
                    .monospace()
                    .size(10.0),
            ));
            dismissal.add(Label::new(
                RichText::new(player_score.1)
                    .color(Color32::WHITE)
                    .monospace()
                    .size(10.0),
            ));
            bowler.add(Label::new(
                RichText::new(player_score.2)
                    .color(Color32::WHITE)
                    .monospace()
                    .size(10.0),
            ));
            bat_score.add(Label::new(
                RichText::new(player_score.3)
                    .color(Color32::WHITE)
                    .monospace()
                    .size(10.0),
            ));
        }
    });
}
fn bowling_scorecard(ui: &mut egui::Ui, scoreboard: &Scoreboard) {
    ui.columns_const(|[name, figures, _extras]| {
        for player in 0..=10 {
            if scoreboard.innings.bowling_team.players[player].return_if_bowled() {
                let player_score = scoreboard
                    .innings
                    .bowling_team
                    .return_player_bowler_score(player);
                name.add(Label::new(
                    RichText::new(player_score.0)
                        .color(Color32::WHITE)
                        .monospace()
                        .size(10.0),
                ));
                figures.add(Label::new(
                    RichText::new(player_score.1)
                        .color(Color32::WHITE)
                        .monospace()
                        .size(10.0),
                ));
            }
        }
    });
}
fn extras_scoreboard(ui: &mut egui::Ui, scoreboard: &Scoreboard) {
    ui.columns_const(|[extra, _, _, team_score]| {
        extra.add(Label::new(
            RichText::new(scoreboard.innings.return_team_extras())
                .color(Color32::WHITE)
                .monospace()
                .size(10.0),
        ));
        team_score.add(Label::new(
            RichText::new("\n".to_string() + &scoreboard.innings.return_team_score())
                .color(Color32::WHITE)
                .monospace()
                .size(10.0),
        ));
    });
}
