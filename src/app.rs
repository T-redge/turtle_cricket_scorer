use crate::{functions::*, player::*, team::*};
use eframe::egui::{self, Align2, Button, Checkbox, Vec2, Ui};
pub struct Scoreboard {
    hide_extra_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
    pub team_1: Team,
    pub team_2: Team,
    batters_picked: bool,
    bowler_picked: bool,
    pub checklist_bat: Vec<bool>,
    checklist_bowl: Vec<bool>,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Scoreboard {
            hide_extra_button: false,
            ball_event: BallEvent::EventWaiting,
            ball_bowled: false,
            team_1: Team::new("Edgewater", TeamRole::BattingTeam),
            team_2: Team::new("Kingsway", TeamRole::BowlingTeam),
            batters_picked: false,
            bowler_picked: false,
            checklist_bat: vec![false; 12],
            checklist_bowl: vec![false; 12],
        }
    }
    pub fn set_hide_button_bool(&mut self) {
        match self.hide_extra_button {
            true => self.hide_extra_button = false,
            false => self.hide_extra_button = true,
        }
    }
    pub fn set_ball_bowled(&mut self) {
        match self.ball_bowled {
            true => self.ball_bowled = false,
            false => self.ball_bowled = true,
        }
    }
    pub fn set_ball_event(&mut self, b_e: BallEvent) {
        self.ball_event = b_e;
    }
    pub fn return_hide_extra_button(&self) -> bool {
        self.hide_extra_button
    }
    pub fn return_ball_event(&self) -> BallEvent {
        self.ball_event
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ball_bowled {
            match_ball_event(self);
        }
        if !self.batters_picked {
            egui::Window::new("Choose Batters")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .default_size([500.0, 300.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.columns_const(|[_ui_1,ui_2,_ui_3]| {
                        get_opening_batters(ui_2, self);
                    });
                });
        } else if !self.bowler_picked {
            egui::Window::new("Choose Bowler")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .default_size([500.0, 300.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.columns_const(|[_ui_1,ui_2,_ui_3]| {
                        get_bowler(ui_2, self);
                    });
                });
        } else {
            egui::TopBottomPanel::bottom("id_buttons").show(ctx, |ui| {
                buttons(ui, self);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                scores(ui, self);
            });
        }
    }
}
pub fn get_opening_batters(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[0],
        scoreboard.team_1.return_player_names(0),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[1],
        scoreboard.team_1.return_player_names(1),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[2],
        scoreboard.team_1.return_player_names(2),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[3],
        scoreboard.team_1.return_player_names(3),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[4],
        scoreboard.team_1.return_player_names(4),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[5],
        scoreboard.team_1.return_player_names(5),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[6],
        scoreboard.team_1.return_player_names(6),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[7],
        scoreboard.team_1.return_player_names(7),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[8],
        scoreboard.team_1.return_player_names(8),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[9],
        scoreboard.team_1.return_player_names(9),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bat[10],
        scoreboard.team_1.return_player_names(10),
    ));
    for x in 0..=10 {
        if scoreboard.checklist_bat[x] {
            scoreboard.team_1.set_player_bat_status(x, PlayerBatStatus::InTheMiddle);
        }
    }
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, Button::new("Select Batters"))
        .clicked()
    {
        scoreboard.batters_picked = true;
    }
}
pub fn get_bowler(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[0],
        scoreboard.team_2.return_player_names(0),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[1],
        scoreboard.team_2.return_player_names(1),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[2],
        scoreboard.team_2.return_player_names(2),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[3],
        scoreboard.team_2.return_player_names(3),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[4],
        scoreboard.team_2.return_player_names(4),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[5],
        scoreboard.team_2.return_player_names(5),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[6],
        scoreboard.team_2.return_player_names(6),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[7],
        scoreboard.team_2.return_player_names(7),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[8],
        scoreboard.team_2.return_player_names(8),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[9],
        scoreboard.team_2.return_player_names(9),
    ));
    ui.add(Checkbox::new(
        &mut scoreboard.checklist_bowl[10],
        scoreboard.team_2.return_player_names(10),
    ));
    for x in 0..=10 {
        if scoreboard.checklist_bowl[x] {
            scoreboard.team_2.set_player_bowl_status(x, PlayerBowlStatus::IsBowling);
        }
    }
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, Button::new("Select Bowler"))
        .clicked()
    {
        scoreboard.bowler_picked = true;
    }
}