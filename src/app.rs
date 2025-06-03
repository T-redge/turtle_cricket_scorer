use crate::{functions::*, player::PlayerStatus, team::*};
use eframe::egui::{self, Align2, Button, Checkbox,Vec2};
pub struct Scoreboard {
    hide_extra_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
    pub team_1: Team,
    pub team_2: Team,
    batters_picked: bool,
    pub checklist_bool: Vec<bool>,
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
            checklist_bool: vec![false; 12],
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
                .anchor(Align2::CENTER_CENTER, [0.0,0.0])
                .default_size([500.0,300.0])
                .movable(false)
                .collapsible(false)
                .show(ctx,|ui|{
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[0], self.team_1.return_player_names(0)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[1], self.team_1.return_player_names(1)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[2], self.team_1.return_player_names(2)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[3], self.team_1.return_player_names(3)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[4], self.team_1.return_player_names(4)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[5], self.team_1.return_player_names(5)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[6], self.team_1.return_player_names(6)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[7], self.team_1.return_player_names(7)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[8], self.team_1.return_player_names(8)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[9], self.team_1.return_player_names(9)));
                    ui.add(Checkbox::new(
                        &mut self.checklist_bool[10], self.team_1.return_player_names(10)));
                    for x in 0..=10 {
                        if self.checklist_bool[x] {
                            self.team_1.set_player_status(x, PlayerStatus::InTheMiddle);
                        }
                    }
                    if ui.add_sized(Vec2{x: 150.0, y:50.0}, Button::new("Select Batters")).clicked() {
                        self.batters_picked = true;
                    }
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

