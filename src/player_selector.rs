use eframe::egui::{self, Button, Checkbox};

use crate::{game::*, team::*};
#[derive(Clone,PartialEq)]
enum PlayerSelection {
    Selected,
    Unselected,
}
pub struct PlayerSelector {
    player_list: Vec<(String,PlayerSelection)>,

    player_bool: Vec<bool>,

    chosen_player_1: String,
    chosen_player_2: String,

    hide_player_selector: bool,

    p_1_select: bool,
}

impl PlayerSelector {
    pub fn new() -> Self {
        Self {
            player_list: vec![(String::new(), PlayerSelection::Unselected);11],

            player_bool: vec![false;11],

            chosen_player_1: String::new(),
            chosen_player_2: String::new(),

            hide_player_selector: false,

            p_1_select: false,
       }
    }
    pub fn show_player_selector(&mut self, ui: &mut egui::Ui, team_name: &str) {
        self.chosen_player_1 = String::new();
        self.chosen_player_2 = String::new();
        for x in 0..load_list(team_name).len() {
            self.player_list[x].0 = load_list(team_name)[x].clone();
        }
        ui.horizontal_wrapped(|ui| { 
            for x in 0..self.player_list.len() {
                ui.add_enabled(true, Checkbox::new(&mut self.player_bool[x], create_game_label(&self.player_list[x].0.clone(),12.0)));
                ui.end_row();
            }
        });
            if ui
                .add_enabled(true, Button::new(create_game_label("Confirm", 20.0)))
                .clicked()
            {
                for x in 0..self.player_list.len() {
                    if self.player_bool[x] {
                        self.player_list[x].1 = PlayerSelection::Selected;
                        self.player_bool[x] = false;
                    }
                }
                for x in 0..self.player_list.len() {
                    if self.player_list[x].1 == PlayerSelection::Selected {
                        if !self.p_1_select {
                            self.chosen_player_1 = self.player_list[x].0.clone();
                            self.player_list[x].1 = PlayerSelection::Unselected;
                            self.p_1_select = true;
                        }   
                    }
                    self.set_hide_player_select(true);
                }
            }
    }
    pub fn set_hide_player_select(&mut self, bool: bool) {
        self.hide_player_selector = bool;
    }
    pub fn return_hide_player_selector(&self) -> bool {
        self.hide_player_selector
    }
    pub fn return_chosen_player1(&self) -> String {
        self.chosen_player_1.clone()
    }
    pub fn return_chosen_player2(&self) -> String {
        self.chosen_player_2.clone()
    }
}
