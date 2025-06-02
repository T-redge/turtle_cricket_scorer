use crate::{functions::*, team::*};
use eframe::egui;
pub struct Scoreboard {
    hide_extra_button: bool,
    ball_event: BallEvent,
    ball_bowled: bool,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Scoreboard {
            hide_extra_button: false,
            ball_event: BallEvent::EventWaiting,
            ball_bowled: false,
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
        let team_1 = Team::new("Edgewater");
        if self.ball_bowled {
            match_ball_event(self);
        }
        egui::TopBottomPanel::bottom("id_buttons").show(ctx, |ui| {
            buttons(ui, self);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            scores(ui, &team_1);
        });
    }
}
