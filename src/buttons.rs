use crate::{
    innings::BallEvent,
    scoreboard::{self, *},
};
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
pub fn wicket_ball_button(ui: &mut Ui, _scoreboard: &mut Scoreboard) {
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
                    {}
                    if ui_2
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("Caught").size(12.0)),
                        )
                        .clicked()
                    {}
                    if ui_3
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("LBW").size(12.0)),
                        )
                        .clicked()
                    {}
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(
                                Vec2 { x: 50.0, y: 20.0 },
                                egui::Button::new(RichText::new("Stumped").size(12.0)),
                            )
                            .clicked()
                        {}
                        if ui_2
                            .add_sized(
                                Vec2 { x: 50.0, y: 20.0 },
                                egui::Button::new(RichText::new("Run Out").size(12.0)),
                            )
                            .clicked()
                        {}
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("5"))
                        .clicked()
                    {
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                        .clicked()
                    {
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
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
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool(false);
                    }
                });
            });
        },
    );
}
pub fn new_over_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, egui::Button::new("New Over"))
        .clicked()
    {
        scoreboard.innings.over_bowled();
    }
}
