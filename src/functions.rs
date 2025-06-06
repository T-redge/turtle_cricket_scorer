use crate::{app::*, player::PlayerStrike, team::*};
use eframe::egui::{self, Color32, Label, RichText, Ui, Vec2};
#[derive(Clone, Copy)]
pub enum BallEvent {
    EventWaiting,
    EventDot,
    EventRuns(RunsEvent),
    EventExtras(ExtrasEvent),
    EventWicket(WicketEvents),
}
#[derive(Clone, Copy)]
pub enum RunsEvent {
    OneScored(u8),
    TwoScored(u8),
    ThreeScored(u8),
    FourScored(u8),
    SixScored(u8),
}
#[derive(Clone, Copy)]
pub enum ExtrasEvent {
    WideBowled(u8),
    NoBallBowled(u8),
    ByesBowled(u8),
    LegbyesBowled(u8),
}
#[derive(Clone, Copy)]
pub enum WicketEvents {
    OutBowled,
    OutCaught,
    OutLBW,
    OutStumped,
    OutRunout,
}

pub fn match_ball_event(scoreboard: &mut Scoreboard) {
    let event = scoreboard.return_ball_event();
    let t1 = &mut scoreboard.team_1;
    let t2 = &mut scoreboard.team_2;
    match event {
        BallEvent::EventWaiting => {}
        BallEvent::EventDot => {
            println!("Dot");
            t1.add_bat_ball_faced();
            t2.add_bowler_ball_bowled();
        }
        BallEvent::EventRuns(ev) => match ev {
            RunsEvent::OneScored(_runs) => {
                println!("One Run");
            }
            RunsEvent::TwoScored(_runs) => {
                println!("Two Runs");
                match_team_roles(t1);
                match_team_roles(t2);
            }
            RunsEvent::ThreeScored(_runs) => {
                println!("Three Runs");
                match_team_roles(t1);
                match_team_roles(t2);
            }
            RunsEvent::FourScored(_runs) => {
                println!("Four Runs");
                match_team_roles(t1);
                match_team_roles(t2);
            }
            RunsEvent::SixScored(_runs) => {
                println!("Six Runs");
                match_team_roles(t1);
                match_team_roles(t2);
            }
        },
        BallEvent::EventExtras(_ev) => {
            println!("Extras");
            match_team_roles(t1);
            match_team_roles(t2);
        }
        BallEvent::EventWicket(_ev) => {
            println!("Wicket");
            match_team_roles(t1);
            match_team_roles(t2);
        }
    }
    scoreboard.set_ball_bowled();
}
//Score Displays
pub fn scores(ui: &mut Ui, scoreboard: &Scoreboard) {
    let t1 = &scoreboard.team_1;
    let t2 = &scoreboard.team_2;
    team_scores(ui, t1);
    batter_scores(ui, t1);
    bowler_scores(ui, t2);
    extra_scores(ui, t1);
}
pub fn team_scores(ui: &mut Ui, team: &Team) {
    let (tm_nm, overs) = team.return_team_score();
    ui.horizontal(|ui| {
        ui.columns_const(|[ui_1, ui_2]| {
            ui_1.add(Label::new(
                RichText::new(tm_nm)
                    .monospace()
                    .color(Color32::WHITE)
                    .size(20.0),
            ));
            ui_2.add(Label::new(
                RichText::new(overs)
                    .monospace()
                    .color(Color32::WHITE)
                    .size(20.0),
            ));
        });
    });
}
pub fn batter_scores(ui: &mut Ui, team: &Team) {
    let (b1, b2) = team.return_player_at_middle();
    ui.add(Label::new(
        RichText::new("\nBatters\n")
            .color(Color32::WHITE)
            .monospace()
            .size(20.0)
            .underline(),
    ));
    ui.columns_const(|[ui_1, ui_2]| {
        ui_1.horizontal_wrapped(|ui| {
            ui.add(Label::new(
                RichText::new(b1.return_player_name())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));
            match b1.return_batter_strike_status() {
                PlayerStrike::OnStrike => {
                    ui.add(Label::new(
                        RichText::new("*").monospace().color(Color32::WHITE),
                    ));
                }
                PlayerStrike::OffStrike => {}
            }
            ui.end_row();
            ui.add(Label::new(
                RichText::new(b2.return_player_name())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));
            match b2.return_batter_strike_status() {
                PlayerStrike::OnStrike => {
                    ui.add(Label::new(
                        RichText::new("*").monospace().color(Color32::WHITE),
                    ));
                }
                PlayerStrike::OffStrike => {}
            }
        });
        ui_2.horizontal_wrapped(|ui| {
            ui.add(Label::new(
                RichText::new(b1.return_player_bat_profile())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));
            ui.end_row();
            ui.add(Label::new(
                RichText::new(b2.return_player_bat_profile())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));
        });
    });
}
pub fn bowler_scores(ui: &mut Ui, team: &Team) {
    let bowler = team.return_player_bowling();
    ui.add(Label::new(
        RichText::new("\nBowlers\n")
            .color(Color32::WHITE)
            .monospace()
            .size(20.0)
            .underline(),
    ));
    ui.columns_const(|[ui_1, ui_2]| {
        ui_1.horizontal_wrapped(|ui| {
            ui.add(Label::new(
                RichText::new(bowler.return_player_name())
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));
            ui.end_row();
            /*ui.add(Label::new(
                RichText::new(b2)
                    .color(Color32::WHITE)
                    .monospace()
                    .size(12.0),
            ));*/
            ui_2.horizontal_wrapped(|ui| {
                ui.add(Label::new(
                    RichText::new(bowler.return_player_bowl_profile())
                        .color(Color32::WHITE)
                        .monospace()
                        .size(12.0),
                ));
                ui.end_row();
                /*ui.add(Label::new(
                    RichText::new(b2_bowl)
                        .color(Color32::WHITE)
                        .monospace()
                        .size(12.0),
                ));*/
            });
        });
    });
}
pub fn extra_scores(ui: &mut Ui, team: &Team) {
    let extras = team.return_extras();
    ui.horizontal(|ui| {
        ui.add(Label::new(
            RichText::new(extras)
                .color(Color32::WHITE)
                .monospace()
                .size(16.0),
        ));
    });
}
//UI Buttons
pub fn buttons(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    ui.columns_const(|[ui_1, ui_2, ui_3, ui_4]| {
        dot_ball_button(ui_1, scoreboard);
        runs_ball_button(ui_2, scoreboard);
        wicket_ball_button(ui_3, scoreboard);
        if scoreboard.return_hide_extra_button() {
            ui_4.columns_const(|[ui_1, ui_2]| {
                wide_ball_button(ui_1, scoreboard);
                noball_ball_button(ui_2, scoreboard);
                bye_ball_button(ui_1, scoreboard);
                legbye_ball_button(ui_2, scoreboard);
            });
        } else {
            extra_ball_button(ui_4, scoreboard);
        }
    })
}
pub fn dot_ball_button(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    if ui
        .add_sized(Vec2 { x: 100.0, y: 50.0 }, egui::Button::new("Dot"))
        .clicked()
    {
        scoreboard.set_ball_event(BallEvent::EventDot);
        scoreboard.set_ball_bowled();
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
                        scoreboard.set_ball_event(BallEvent::EventRuns(RunsEvent::OneScored(1)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_ball_event(BallEvent::EventRuns(RunsEvent::TwoScored(2)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_ball_event(BallEvent::EventRuns(RunsEvent::ThreeScored(3)));
                        scoreboard.set_ball_bowled();
                    }
                });
                ui.centered_and_justified(|ui| {
                    ui.columns_const(|[ui_1, ui_2]| {
                        if ui_1
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                            .clicked()
                        {
                            scoreboard
                                .set_ball_event(BallEvent::EventRuns(RunsEvent::FourScored(4)));
                            scoreboard.set_ball_bowled();
                        }
                        if ui_2
                            .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                            .clicked()
                        {
                            scoreboard
                                .set_ball_event(BallEvent::EventRuns(RunsEvent::SixScored(6)));
                            scoreboard.set_ball_bowled();
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
                        scoreboard.set_ball_event(BallEvent::EventWicket(WicketEvents::OutBowled));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("Caught").size(12.0)),
                        )
                        .clicked()
                    {
                        scoreboard.set_ball_event(BallEvent::EventWicket(WicketEvents::OutCaught));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_3
                        .add_sized(
                            Vec2 { x: 50.0, y: 20.0 },
                            egui::Button::new(RichText::new("LBW").size(12.0)),
                        )
                        .clicked()
                    {
                        scoreboard.set_ball_event(BallEvent::EventWicket(WicketEvents::OutLBW));
                        scoreboard.set_ball_bowled();
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
                                .set_ball_event(BallEvent::EventWicket(WicketEvents::OutStumped));
                            scoreboard.set_ball_bowled();
                        }
                        if ui_2
                            .add_sized(
                                Vec2 { x: 50.0, y: 20.0 },
                                egui::Button::new(RichText::new("Run Out").size(12.0)),
                            )
                            .clicked()
                        {
                            scoreboard
                                .set_ball_event(BallEvent::EventWicket(WicketEvents::OutRunout));
                            scoreboard.set_ball_bowled();
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
        scoreboard.set_hide_button_bool();
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
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::WideBowled(1)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::WideBowled(2)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::WideBowled(3)));
                        scoreboard.set_ball_bowled();
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::WideBowled(4)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("5"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::WideBowled(5)));
                        scoreboard.set_ball_bowled();
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
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::NoBallBowled(1)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::NoBallBowled(2)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_3
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::NoBallBowled(3)));
                        scoreboard.set_ball_bowled();
                    }
                });
            });
            ui.centered_and_justified(|ui| {
                ui.columns_const(|[ui_1, ui_2]| {
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::NoBallBowled(4)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("6"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::NoBallBowled(6)));
                        scoreboard.set_ball_bowled();
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
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::ByesBowled(1)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::ByesBowled(2)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::ByesBowled(3)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::ByesBowled(4)));
                        scoreboard.set_ball_bowled();
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
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(1)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("2"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(2)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_1
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("3"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(3)));
                        scoreboard.set_ball_bowled();
                    }
                    if ui_2
                        .add_sized(Vec2 { x: 25.0, y: 20.0 }, egui::Button::new("4"))
                        .clicked()
                    {
                        scoreboard.set_hide_button_bool();
                        scoreboard
                            .set_ball_event(BallEvent::EventExtras(ExtrasEvent::LegbyesBowled(4)));
                        scoreboard.set_ball_bowled();
                    }
                });
            });
        },
    );
}

pub fn match_team_roles(team: &mut Team) {
    match team.return_team_role() {
        TeamRole::BattingTeam => {}
        TeamRole::BowlingTeam => {}
    }
}
