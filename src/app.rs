use crate::{functions::*, player::*, team::*};
use eframe::egui::{self, Align2, Button, Checkbox, Ui, Vec2};
pub struct Scoreboard {
    hide_extra_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
    over_bowled: bool,
    pub team_1: Team,
    pub team_2: Team,
    player_strike: bool,
    batters_picked: bool,
    bowler_picked: bool,
    checklist_bat: Vec<bool>,
    selected_bowler: usize,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Scoreboard {
            hide_extra_button: false,
            ball_event: BallEvent::EventWaiting,
            ball_bowled: false,
            over_bowled: false,
            team_1: Team::new("Edgewater", TeamRole::BattingTeam),
            team_2: Team::new("Kingsway", TeamRole::BowlingTeam),
            player_strike: false,
            batters_picked: false,
            bowler_picked: false,
            checklist_bat: vec![false; 12],
            selected_bowler: 0,
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
    pub fn check_over_bowled(&mut self) {
        if self.team_2.return_team_balls_bowled() == 6 {
            self.over_bowled = true;
            self.bowler_picked = false;
        }
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
            self.check_over_bowled();
        }
        if !self.batters_picked {
            egui::Window::new("Select Openers")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    get_openers(ui, self);
                });
        } else if !self.player_strike {
            egui::Window::new("Select Batter on Strike")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    set_batter_strike(ui, self);
                });
        } else if !self.bowler_picked || self.over_bowled {
            choose_bowler_window(ctx, self);
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
pub fn set_batter_strike(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    let (b1, b2) = scoreboard.team_1.return_player_at_middle_usize();
    ui.columns_const(|[col_1, col_2]| {
        if col_1
            .add_sized(
                Vec2 { x: 100.0, y: 50.0 },
                Button::new(scoreboard.team_1.return_player_names(b1)),
            )
            .clicked()
        {
            scoreboard
                .team_1
                .set_batter_strike(b1, PlayerStrike::OnStrike);
            scoreboard.player_strike = true;
        }
        if col_2
            .add_sized(
                Vec2 { x: 100.0, y: 50.0 },
                Button::new(scoreboard.team_1.return_player_names(b2)),
            )
            .clicked()
        {
            scoreboard
                .team_1
                .set_batter_strike(b2, PlayerStrike::OnStrike);
            scoreboard.player_strike = true;
        }
    });
}
pub fn get_openers(ui: &mut Ui, scoreboard: &mut Scoreboard) {
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
            scoreboard
                .team_1
                .set_player_bat_status(x, PlayerBatStatus::InTheMiddle);
        }
    }
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, Button::new("Select Openers"))
        .clicked()
    {
        scoreboard.batters_picked = true;
    }
}
pub fn get_bowler(ui: &mut Ui, scoreboard: &mut Scoreboard) {
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(0),
        scoreboard.team_2.return_player_names(0),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(1),
        scoreboard.team_2.return_player_names(1),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(2),
        scoreboard.team_2.return_player_names(2),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(3),
        scoreboard.team_2.return_player_names(3),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(4),
        scoreboard.team_2.return_player_names(4),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(5),
        scoreboard.team_2.return_player_names(5),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(6),
        scoreboard.team_2.return_player_names(6),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(7),
        scoreboard.team_2.return_player_names(7),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(8),
        scoreboard.team_2.return_player_names(8),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(9),
        scoreboard.team_2.return_player_names(9),
    );
    ui.radio_value(
        &mut scoreboard
            .team_2
            .return_player_names(scoreboard.selected_bowler),
        scoreboard.team_2.return_player_names(10),
        scoreboard.team_2.return_player_names(10),
    );
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, Button::new("Select Bowler"))
        .clicked()
    {
        scoreboard.bowler_picked = true;
    }
}
pub fn choose_bowler_window(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Choose Bowler")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .default_size([500.0, 300.0])
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.columns_const(|[_ui_1, ui_2, _ui_3]| {
                get_bowler(ui_2, scoreboard);
            });
        });
}
