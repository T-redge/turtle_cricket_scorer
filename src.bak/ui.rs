use crate::{app::Scoreboard, over::*};
use eframe::egui::{self, Align2, Color32, Label, RichText, Ui, Vec2};

pub fn print_end_over(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("End Over")
        .collapsible(false)
        .fixed_size([400.0, 150.0])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let mut over_label = String::from("Over: ");
                over_label.push_str(&(scoreboard.innings.return_over_count() + 1).to_string());
                ui.add(Label::new(
                    RichText::new(over_label)
                        .color(Color32::WHITE)
                        .size(20.0)
                        .monospace(),
                ));
                let mut runs_label = String::from(" Runs: ");
                runs_label.push_str(&scoreboard.innings.current_over.return_runs_conceded().to_string());
                ui.add(Label::new(
                    RichText::new(runs_label)
                        .color(Color32::WHITE)
                        .size(20.0)
                        .monospace(),
                ));
                let mut wicket_label = String::from(" Wickets: ");
                wicket_label.push_str(&scoreboard.innings.current_over.return_wickets_taken().to_string());
                ui.add(Label::new(
                    RichText::new(wicket_label)
                        .color(Color32::WHITE)
                        .size(20.0)
                        .monospace(),
                ));
                ui.end_row();
                ui.add(Label::new(
                    RichText::new(&scoreboard.innings.current_over.return_ball_history())
                        .color(Color32::WHITE)
                        .size(16.0)
                        .monospace(),
                ));
                ui.allocate_space(ui.available_size());
            });
            ui.centered_and_justified(|ui| {
                if ui
                    .add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("New Over"))
                    .clicked()
                {
                    scoreboard.innings.commit_to_overhistory();
                    scoreboard.innings.add_over_count();
                    scoreboard.innings.current_over = Over::new();
                    scoreboard.pick_bowler = true;
                }
            });
        });
}
pub fn display_teams(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Teams")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .open(&mut scoreboard.hide_teamlist)
        .show(ctx, |ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    ui_1.add(egui::Label::new(
                        RichText::new(&scoreboard.innings.home_team.team_name)
                            .size(24.0)
                            .monospace()
                            .color(Color32::WHITE)
                            .underline(), 
                    ));
                    ui_1.add(egui::Label::new(&scoreboard.innings.return_home_team()));
                    ui_2.add(egui::Label::new(
                        RichText::new(&scoreboard.innings.away_team.team_name)
                            .monospace()
                            .size(24.0)
                            .color(Color32::WHITE)
                            .underline(),
                    ));
                    ui_2.add(egui::Label::new(&scoreboard.innings.return_away_team()));
                });
            });
}



pub fn dot_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    if ui
        .add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Dot"))
        .clicked()
    {
        scoreboard.innings.current_over.ball_event = BallEvent::EventDot;
        scoreboard.innings.current_over.change_ball_flag(true);
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
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventRuns(RunsEvent::OneScored(1));
                            scoreboard.innings.current_over.change_ball_flag(true);
                            scoreboard.innings.add_score(1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventRuns(RunsEvent::TwoScored(2));
                            scoreboard.innings.current_over.change_ball_flag(true);
                            scoreboard.innings.add_score(2);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventRuns(RunsEvent::ThreeScored(3));
                            scoreboard.innings.current_over.change_ball_flag(true);
                            scoreboard.innings.add_score(3);
                    }
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                            .clicked()
                        {
                            scoreboard.innings.current_over.ball_event =
                                BallEvent::EventRuns(RunsEvent::FourScored(4));
                                scoreboard.innings.current_over.change_ball_flag(true);
                                scoreboard.innings.add_score(4);
                        }
                        if ui_2
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                            .clicked()
                        {
                            scoreboard.innings.current_over.ball_event =
                                BallEvent::EventRuns(RunsEvent::SixScored(6));
                                scoreboard.innings.current_over.change_ball_flag(true);
                                scoreboard.innings.add_score(6);
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
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("Bowled"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventWicket(WicketEvents::OutBowled);
                            scoreboard.innings.current_over.change_ball_flag(true);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("Caught"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventWicket(WicketEvents::OutCaught);
                            scoreboard.innings.current_over.change_ball_flag(true);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("LBW"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventWicket(WicketEvents::OutLBW);
                            scoreboard.innings.current_over.change_ball_flag(true);
                    }
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("Stumped"))
                            .clicked()
                        {
                            scoreboard.innings.current_over.ball_event =
                                BallEvent::EventWicket(WicketEvents::OutStumped);
                                scoreboard.innings.current_over.change_ball_flag(true);
                        }
                        if ui_2
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("Run Out"))
                            .clicked()
                        {
                            scoreboard.innings.current_over.ball_event =
                                BallEvent::EventWicket(WicketEvents::OutRunout);
                                scoreboard.innings.current_over.change_ball_flag(true);
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
        scoreboard.extra_hidden = true;
    }
}
pub fn wide_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let wide_button = ui.add_sized(Vec2 { x: 100.0, y: 23.5 }, egui::Button::new("Wide"));
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
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::WideBowled(1));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(1);
                        scoreboard.innings.extras.add_wide(1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::WideBowled(2));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(2);
                        scoreboard.innings.extras.add_wide(2);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::WideBowled(3));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(3);
                        scoreboard.innings.extras.add_wide(3);
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::WideBowled(4));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(4);
                        scoreboard.innings.extras.add_wide(4);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("5"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::WideBowled(5));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(5);
                        scoreboard.innings.extras.add_wide(5);
                    }
                });
            });
        },
    );
}
pub fn noball_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let noball_button = ui.add_sized(Vec2 { x: 100.0, y: 23.5 }, egui::Button::new("Noball"));
    let popup_id = ui.make_persistent_id("Id_Noball");
    if noball_button.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        scoreboard.innings.extras.add_noball();
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
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::NoBallBowled(1));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(1 + 1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::NoBallBowled(2));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(2 + 1);
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::NoBallBowled(3));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(3 + 1);
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::NoBallBowled(4));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(4 + 1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::NoBallBowled(6));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(1 + 1);
                    }
                });
            });
        },
    );
}
pub fn bye_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let bye_button = ui.add_sized(Vec2 { x: 100.0, y: 23.5 }, egui::Button::new("Byes"));
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
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::ByesBowled(1));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(1);
                        scoreboard.innings.extras.add_byes(1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::ByesBowled(2));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(2);
                        scoreboard.innings.extras.add_byes(2);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::ByesBowled(3));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(3);
                        scoreboard.innings.extras.add_byes(3);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::ByesBowled(4));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(4);
                        scoreboard.innings.extras.add_byes(4);
                    }
                });
            });
        },
    );
}
pub fn legbye_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let legbye_button = ui.add_sized(Vec2 { x: 100.0, y: 23.5 }, egui::Button::new("Legbyes"));
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
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(1));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(1);
                        scoreboard.innings.extras.add_legbyes(1);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(2));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(2);
                        scoreboard.innings.extras.add_legbyes(2);
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(3));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(3);
                        scoreboard.innings.extras.add_legbyes(3);
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.innings.current_over.ball_event =
                            BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(4));
                        scoreboard.innings.current_over.change_ball_flag(true);
                        scoreboard.extra_hidden = false;
                        scoreboard.innings.add_score(4);
                        scoreboard.innings.extras.add_legbyes(4);
                    }
                });
            });
        },
    );
}
