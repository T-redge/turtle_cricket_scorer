use crate::{innings::BallEvent, scoreboard::*};
use eframe::egui::{self, RichText, Ui, Vec2};

pub fn dot_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    if ui
        .add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Dot"))
        .clicked()
    {
        scoreboard.innings.set_ball_event(BallEvent::DotBall);
    }
}
pub fn runs_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let run_button = ui.add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Runs"));
    let popup_id = ui.make_persistent_id("Id_Runs");
    if run_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &run_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2, ui_3]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("1"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::RunScored(1));
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::RunScored(2));
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::RunScored(3));
                    }
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                            .clicked()
                        {
                            scoreboard.innings.set_ball_event(BallEvent::RunScored(4));
                        }
                        if ui_2
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                            .clicked()
                        {
                            scoreboard.innings.set_ball_event(BallEvent::RunScored(6));
                        }
                    });
                });
            });
        },
    );
}
pub fn wicket_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let wicket_button = ui.add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Wicket"));
    let popup_id = ui.make_persistent_id("Id_Wicket");
    if wicket_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &wicket_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2, ui_3]| {
                    if ui_1
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("Bowled").size(12.0)),
                        )
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::WicketBowler("Bowled"));
                    }
                    if ui_2
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("Caught").size(12.0)),
                        )
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::WicketBowler("Caught"));

                    }
                    if ui_3
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("LBW").size(12.0)),
                        )
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::WicketBowler("Lbw"));
                    }
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(
                                Vec2 { x: 50.0, y: 20.0 },
                                egui::Button::new(RichText::new("Stumped").size(12.0)),
                            )
                            .clicked()
                        {
                            scoreboard
                                .innings
                                .set_ball_event(BallEvent::WicketBowler("Stumped"));
                        }
                        if ui_2
                            .add_sized(
                                Vec2 { x: 50.0, y: 20.0 },
                                egui::Button::new(RichText::new("Run Out").size(12.0)),
                            )
                            .clicked()
                        {
                            scoreboard
                                .innings
                                .set_ball_event(BallEvent::WicketTeam("Run Out"));

                        }
                    });
                });
            });
        },
    );
}
pub fn extra_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    if ui
        .add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Extra"))
        .clicked()
    {
        scoreboard.set_hide_button_bool(true);
    }
}
pub fn wide_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let wide_button = ui.add_sized(Vec2 { x: 50.0, y: 23.5 }, egui::Button::new("Wide"));
    let popup_id = ui.make_persistent_id("Id_Wides");
    if wide_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &wide_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2, ui_3]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("1"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::WideBowled(1));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::WideBowled(2));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::WideBowled(3));
                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::WideBowled(4));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("5"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::WideBowled(5));
                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
        },
    );
}
pub fn noball_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let noball_button = ui.add_sized(Vec2 { x: 50.0, y: 23.5 }, egui::Button::new("Noball"));
    let popup_id = ui.make_persistent_id("Id_Noball");
    if noball_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &noball_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2, ui_3]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("1"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::NoballBowled(1));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::NoballBowled(2));

                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::NoballBowled(3));

                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::NoballBowled(4));

                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::NoballBowled(6));

                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
        },
    );
}
pub fn bye_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let bye_button = ui.add_sized(Vec2 { x: 50.0, y: 23.5 }, egui::Button::new("Byes"));
    let popup_id = ui.make_persistent_id("Id_Bye");
    if bye_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &bye_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("1"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::ByeBowled(1));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::ByeBowled(2));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::ByeBowled(3));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.set_ball_event(BallEvent::ByeBowled(4));
                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
        },
    );
}
pub fn legbye_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let legbye_button = ui.add_sized(Vec2 { x: 50.0, y: 23.5 }, egui::Button::new("Legbyes"));
    let popup_id = ui.make_persistent_id("Id_Legbye");
    if legbye_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    let below = egui::AboveOrBelow::Above;
    let close_on_click = egui::popup::PopupCloseBehavior::CloseOnClick;
    egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &legbye_button,
        below,
        close_on_click,
        |ui| {
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("1"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::LegbyeBowled(1));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::LegbyeBowled(2));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::LegbyeBowled(3));
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard
                            .innings
                            .set_ball_event(BallEvent::LegbyeBowled(4));
                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
        },
    );
}
pub fn new_over_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    ui.vertical_centered_justified(|ui| {
        if ui
            .add_sized(Vec2 { x: 150.0, y: 50.0 }, egui::Button::new("New Over"))
            .clicked()
        {
            scoreboard.innings.over_bowled();
            let last_over = scoreboard.innings.bowling_team.return_last_over_bowler();
            if last_over < 11 {
                scoreboard.innings.bowling_team.players[last_over]
                    .set_bowler_status(crate::player::BowlingStatus::Waiting);
            }
            let bowler_over = scoreboard.innings.bowling_team.return_current_bowler();
            scoreboard.innings.bowling_team.players[bowler_over]
                .set_bowler_status(crate::player::BowlingStatus::BowledLastOver);
            scoreboard.set_bowler_picked(false);
            scoreboard.innings.reset_over_totals();
        }
    });
}
