use std::fs;

use eframe::egui;

pub struct TeamSelector {
    team_list: Vec<String>,
    team: String,

    team_selected: bool,
}

impl TeamSelector {
    pub fn new() -> Self {
        Self {
            team_list: create_teamlist_vec(),
            team: String::from("Team"),

            team_selected: false,
        }
    }
    pub fn show_team_select(&mut self, ui: &mut egui::Ui, id: &'static str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{}", &self.team))
            .show_ui(ui, |ui| {
                for t in 0..self.team_list.len() {
                    let value = ui.selectable_value(
                        &mut &self.team_list[t],
                        &self.team,
                        &self.team_list[t],
                    );

                    if value.clicked() {
                        self.team = self.team_list[t].clone();
                        self.set_team_selected(true);
                    }
                }
            });
    }

    pub fn return_team(&self) -> String {
        self.team.clone()
    }
    pub fn return_team_selected(&self) -> bool {
        self.team_selected
    }
    fn set_team_selected(&mut self, bool: bool) {
        self.team_selected = bool;
    }
}

fn create_teamlist_vec() -> Vec<String> {
    let mut tmp_list: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir("./teams") {
        for entry in entries {
            if let Ok(entry) = entry {
                tmp_list.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    tmp_list
}
