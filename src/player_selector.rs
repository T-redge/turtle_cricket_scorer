use eframe::egui::{self, Button, Checkbox};

use crate::{game::*, team::*};

pub struct PlayerSelector {
    player_list: Vec<String>,

    player_bool: Vec<bool>,

    chosen_player_1: String,
    chosen_player_2: String,

    hide_player_selector: bool,

    num_chosen: u8,

    player_1_picked: bool,
    player_2_picked: bool,
}

impl PlayerSelector {
    pub fn new() -> Self {
        Self {
            player_list: Vec::new(),

            player_bool: vec![false;11],

            chosen_player_1: String::new(),
            chosen_player_2: String::new(),

            hide_player_selector: false,

            num_chosen: 0,

            player_1_picked: false,
            player_2_picked: false,
        }
    }
    pub fn show_player_selector(&mut self, ui: &mut egui::Ui, team_name: &str, num_players: u8) {
        self.player_list = load_list(team_name);
        ui.horizontal_wrapped(|ui| { 
            for x in 0..self.player_list.len() {
                if self.num_chosen == num_players {
                    ui.add_enabled(false, Checkbox::new(&mut self.player_bool[x], create_game_label(&self.player_list[x].clone(),12.0)));
                } else {
                     ui.add_enabled(true, Checkbox::new(&mut self.player_bool[x], create_game_label(&self.player_list[x].clone(),12.0)));
                }
                ui.end_row();
            }
        });
        ui.horizontal_wrapped(|ui| {
            let button_enabled = if self.player_1_picked && self.player_2_picked {
                    true
                } else {
                    false
                };
            if ui
                .add_enabled(button_enabled, Button::new(create_game_label("Confirm", 20.0)))
                .clicked()
            {
                let mut tmp = String::new();
                for x in 0..self.player_list.len() {
                    if self.player_bool[x] {
                        tmp = self.player_list[x].clone();
                    }
                }
                self.chosen_player_1 = tmp;
                self.set_hide_player_select(true);
            }
        });
    }
    fn set_hide_player_select(&mut self, bool: bool) {
        self.hide_player_selector = bool;
    }
    pub fn return_hide_player_selector(&self) -> bool {
        self.hide_player_selector
    }
    pub fn return_chosen_player(&self) -> String {
        self.chosen_player_1.clone()
    }
}
