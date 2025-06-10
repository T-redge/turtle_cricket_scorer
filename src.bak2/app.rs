use crate::{functions::*, player::*, team::*};
use eframe::egui::{self, Align2, Button, Checkbox, Label, Ui, Vec2};
pub struct Scoreboard {
    hide_extra_button: bool,
    hide_over_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
    pub team_1: Team,
    pub team_2: Team,
    player_strike: bool,
    batters_picked: bool,
    pub open_bowler_picked: bool,
    bowler_picked: bool,
    selected_bat: Vec<bool>,
    selected_bowler: String,
    player_number: Vec<usize>,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Scoreboard {
            hide_extra_button: false,
            hide_over_button: true,
            ball_event: BallEvent::EventWaiting,
            ball_bowled: false,
            team_1: Team::new("Edgewater", TeamRole::BattingTeam),
            team_2: Team::new("Kingsway", TeamRole::BowlingTeam),
            player_strike: false,
            batters_picked: false,
            open_bowler_picked: false,
            bowler_picked: false,
            selected_bat: vec![false; 12],
            selected_bowler: String::new(),
            player_number: vec![0,1,2,3,4,5,6,7,8,9,10],
        }
    }
    pub fn set_bowler_picked(&mut self) {
        match self.bowler_picked {
            false => self.bowler_picked = true,
            true => self.bowler_picked = false,
        }
    }
    pub fn set_hide_button_bool(&mut self) {
        match self.hide_extra_button {
            true => self.hide_extra_button = false,
            false => self.hide_extra_button = true,
        }
    }
    pub fn set_over_button_bool(&mut self) {
        match self.hide_over_button {
            true => self.hide_over_button = false,
            false => self.hide_over_button = true,
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
            self.set_over_button_bool();
           
        }
    }
    pub fn return_hide_extra_button(&self) -> bool {
        self.hide_extra_button
    }
    pub fn return_ball_event(&self) -> BallEvent {
        self.ball_event
    }
    pub fn return_opening_bowler_bool(&self) -> bool {
        self.open_bowler_picked
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        debug_window(self, ctx);
        if self.ball_bowled {
            match_ball_event(self);
            self.check_over_bowled();
        }
        if !self.batters_picked {
            choose_batter_window(ctx, self);
        } else if !self.player_strike {
            choose_batter_strike(ctx, self);
        } else if !self.bowler_picked {
            choose_bowler_window(ctx, self);
        } else {
            egui::TopBottomPanel::bottom("id_buttons").show(ctx, |ui| {
                if !self.hide_over_button {
                    new_over_button(ui, self);
                } else {
                    buttons(ui, self);
                }
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
    for player in &scoreboard.player_number {
    ui.add(
        Checkbox::new(
            &mut scoreboard.selected_bat[*player],
            scoreboard.team_1.return_player_names(*player)));
    }
    for player in &scoreboard.player_number {
        if scoreboard.selected_bat[*player] {
            scoreboard
                .team_1
                .set_player_bat_status(*player, PlayerBatStatus::InTheMiddle);
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
    for x in &scoreboard.player_number {
        ui.radio_value(
            &mut scoreboard.selected_bowler,
            scoreboard.team_2.return_player_names(scoreboard.player_number[*x]).to_string(),
            scoreboard.team_2.return_player_names(scoreboard.player_number[*x]));
        }
    if ui
        .add_sized(Vec2 { x: 150.0, y: 50.0 }, Button::new("Select Bowler"))
        .clicked()
    { 
        for x in &scoreboard.player_number{
            if scoreboard.selected_bowler == scoreboard.team_2.return_player_names(scoreboard.player_number[*x]) {
                scoreboard.team_2.set_player_bowl_status(*x, PlayerBowlStatus::IsBowling);
            }
        }
        scoreboard.set_bowler_picked();
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
pub fn choose_batter_window(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Select Openers")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    get_openers(ui, scoreboard);
                });
}
pub fn choose_batter_strike(ctx: &egui::Context, scoreboard: &mut Scoreboard) {
    egui::Window::new("Select Batter on Strike")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .movable(false)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    set_batter_strike(ui, scoreboard);
                });
}
pub fn debug_window(scoreboard: &Scoreboard, ctx: &egui::Context) {
    egui::Window::new("Debug").show(ctx, |ui| {
        let bat_p = scoreboard.batters_picked.to_string();
        let bowl_p = scoreboard.bowler_picked.to_string();

        let bowl_status1 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(0)));
        let bowl_status2 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(1)));
        let bowl_status3 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(2)));
        let bowl_status4 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(3)));
        let bowl_status5 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(4)));
        let bowl_status6 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(5)));
        let bowl_status7 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(6)));
        let bowl_status8 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(7)));
        let bowl_status9 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(8)));
        let bowl_status10 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(9)));
        let bowl_status11 = egui::RichText::new(format!("{:?}",scoreboard.team_2.return_player_bowl_status(10)));

        ui.add(Label::new(bat_p));
        ui.add(Label::new(bowl_p));
        ui.add(Label::new(bowl_status1));
        ui.add(Label::new(bowl_status2));
        ui.add(Label::new(bowl_status3));
        ui.add(Label::new(bowl_status4));
        ui.add(Label::new(bowl_status5));
        ui.add(Label::new(bowl_status6));
        ui.add(Label::new(bowl_status7));
        ui.add(Label::new(bowl_status8));
        ui.add(Label::new(bowl_status9));
        ui.add(Label::new(bowl_status10));
        ui.add(Label::new(bowl_status11));
    });
}