use crate::{innings::*, ui::*};
use eframe::egui;
#[derive(Clone)]
pub struct Scoreboard {
    pub innings: Innings,
    pub extra_hidden: bool,
    pub hide_teamlist: bool,
    pub pick_bowler: bool,
    pub check_bool: bool,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Scoreboard {
            innings: Innings::new(),
            extra_hidden: false,
            hide_teamlist: true,
            pick_bowler: false,
            check_bool: false,
        }
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        display_teams(ctx, self);
        egui::TopBottomPanel::top("Scoreboard").show(ctx, |ui| {
            ui.add(self.innings.clone());
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.innings.current_over.return_ball_flag() {
                self.innings.current_over.ball_bowled();
            }
            if self.innings.end_over() {
                print_end_over(ctx, self);
            }

            if !self.innings.end_over() {
                ui.columns_const(|[ui_1, ui_2]| {
                    dot_ball_button(ui_1, self);
                    runs_ball_button(ui_2, self);
                    wicket_ball_button(ui_2, self);
                    if !self.extra_hidden {
                        extra_ball_button(ui_1, self);
                    }
                    if self.extra_hidden {
                        ui_1.columns_const(|[ui_1, ui_2]| {
                            wide_ball_button(ui_1, self);
                            noball_ball_button(ui_2, self);
                            bye_ball_button(ui_1, self);
                            legbye_ball_button(ui_2, self);
                        });
                    }
                });
            }
        });
    }
}
