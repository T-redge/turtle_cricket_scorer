use crate::{functions::*, player::*, team::*};
use eframe::egui::{self, Align2, Button, Checkbox, ComboBox, Ui, Vec2};
#[derive(Debug,PartialEq)]
pub enum Players {
    None,
    BatOne(String),
    BatTwo(String),
    BatThree(String),
    BatFour(String),
    BatFive(String),
    BatSix(String),
    BatSeven(String),
    BatEight(String),
    BatNine(String),
    BatTen(String),
    BatEleven(String)
}
impl Players {
    pub fn name(&self) -> String {
        match self {
            Self::None => "Choose".to_string(),
            Self::BatOne(name) => name.clone(),
            Self::BatTwo(name) => name.clone(),
            Self::BatThree(name) => name.clone(),
            Self::BatFour(name) => name.clone(),
            Self::BatFive(name) => name.clone(),
            Self::BatSix(name) => name.clone(),
            Self::BatSeven(name) => name.clone(),
            Self::BatEight(name) => name.clone(),
            Self::BatNine(name) => name.clone(),
            Self::BatTen(name) => name.clone(),
            Self::BatEleven(name) => name.clone(),

        }
    }
}

pub struct Scoreboard {
    hide_extra_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
    pub team_1: Team,
    pub team_2: Team,
    batter_one_picked: bool,
    batter_selected: String,
    //batter_two_picked: bool,
    player_strike: bool,
    bowler_picked: bool,
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
            batter_one_picked: false,
            batter_selected: "New".to_string(),
            //batter_two_picked: false,
            player_strike: false,
            bowler_picked: false,
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
        if !self.batter_one_picked {
             egui::Window::new("Select Batter")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {

                egui::ComboBox::new("B1", "Select Batter")
                    .selected_text(format!("{}",self.batter_selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatOne(
                                self.team_1.return_player_names(0).to_string()).name(),
                            self.team_1.return_player_names(0));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatTwo(
                                self.team_1.return_player_names(1).to_string()).name(),
                            self.team_1.return_player_names(1));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatThree(
                                self.team_1.return_player_names(2).to_string()).name(),
                            self.team_1.return_player_names(2));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatFour(
                                self.team_1.return_player_names(3).to_string()).name(),
                            self.team_1.return_player_names(3));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatFive(
                                self.team_1.return_player_names(4).to_string()).name(),
                            self.team_1.return_player_names(4));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatSix(
                                self.team_1.return_player_names(5).to_string()).name(),
                            self.team_1.return_player_names(5));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatSeven(
                                self.team_1.return_player_names(6).to_string()).name(),
                            self.team_1.return_player_names(6));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatEight(
                                self.team_1.return_player_names(7).to_string()).name(),
                            self.team_1.return_player_names(7));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatNine(
                                self.team_1.return_player_names(8).to_string()).name(),
                            self.team_1.return_player_names(8));
                                ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatTen(
                                self.team_1.return_player_names(9).to_string()).name(),
                            self.team_1.return_player_names(9));
                            ui.selectable_value(
                            &mut self.batter_selected,
                            Players::BatEleven(
                                self.team_1.return_player_names(10).to_string()).name(),
                            self.team_1.return_player_names(10));
                    });
                });
        }else if !self.player_strike {
            egui::Window::new("Select Batter on Strike")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    set_batter_strike(ui, self);
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
pub fn get_batters(ui: &mut Ui, scoreboard: &mut Scoreboard) {
}
pub fn set_batter_strike(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let (b1,b2) = scoreboard.team_1.return_player_at_middle_usize();
    ui.columns_const(|[col_1,col_2]| {
        if col_1.add_sized(Vec2{x:100.0, y: 50.0}, Button::new(scoreboard.team_1.return_player_names(b1))).clicked() {
            scoreboard.team_1.set_batter_strike(b1, PlayerStrike::OnStrike);
            scoreboard.player_strike = true;
        }
        if col_2.add_sized(Vec2{x:100.0, y: 50.0}, Button::new(scoreboard.team_1.return_player_names(b2))). clicked() {
            scoreboard.team_1.set_batter_strike(b2, PlayerStrike::OnStrike);
            scoreboard.player_strike = true;
        }
    });
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