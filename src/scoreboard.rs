use crate::{buttons::*, innings::*};
use eframe::egui::{self, Align2, Button, Color32, Label, RichText};
pub struct Scoreboard {
    pub innings: Innings,
    innings_started: bool,
    extra_button_hidden: bool,
}

impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.innings_started {
            team_lists(ctx, self);
        } else if !self.innings.check_innings_finished() {
            if self.innings.check_over_finished() {
                over_recap(ctx, self);
            }
            team_scores(ctx, self);
            batter_scores(ctx, self);
            bowler_scores(ctx, self);
            extra_scores(ctx, self);
            button_bar(ctx, self);
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
        }
    }
    pub fn set_hide_button_bool(&mut self, set: bool) {
        self.extra_button_hidden = set;
    }
    pub fn return_hide_button_bool(&self) -> bool {
        self.extra_button_hidden
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
                        RichText::new(String::from("0/0"))
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
            let bat_1 = scoreboard
                .innings
                .batting_team
                .return_player_batter_score(0);
            let bat_2 = scoreboard
                .innings
                .batting_team
                .return_player_batter_score(1);

            ui.columns_const(|[ui_1, ui_2]| {
                ui_1.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(bat_1.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(bat_2.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                });
                ui_2.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(bat_1.1)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(bat_2.1)
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
            let bowl_1 = scoreboard
                .innings
                .bowling_team
                .return_player_bowler_score(0);
            let bowl_2 = scoreboard
                .innings
                .bowling_team
                .return_player_bowler_score(1);
            ui.columns_const(|[ui_1, ui_2]| {
                ui_1.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(bowl_1.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(bowl_2.0)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                });
                ui_2.horizontal_wrapped(|ui| {
                    ui.add(Label::new(
                        RichText::new(bowl_1.1)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                    ui.end_row();
                    ui.add(Label::new(
                        RichText::new(bowl_2.1)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(12.0),
                    ));
                });
            });
        });
}
fn extra_scores(ctx: &egui::Context, _scoreboard: &Scoreboard) {
    egui::TopBottomPanel::top("id_Extra_Scores")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new("\nExtras ( Wd: 0, Nb: 0, B: 0, Lb: 0 )\n")
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
fn innings_recap(ctx: &egui::Context, _scoreboard: &Scoreboard) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("End of Innings!");
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
            new_over_button(ui, scoreboard);
        });
}
