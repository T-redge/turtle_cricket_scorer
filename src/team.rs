use std::{fs::File, io::*};

use crate::app::TeamRoles;

pub struct Team {
    name: String,

    role: TeamRoles,

    player_list: Vec<String>,
}

impl Team {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),

            role: TeamRoles::Waiting,
            
            player_list: Vec::new(),
        }
    }
    pub fn set_team_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_team_vec(&mut self, name: &str) {
        self.player_list = load_list(name);
    }
    pub fn set_team_role(&mut self, role: TeamRoles) {
        self.role = role;
    }
    pub fn return_team_name(&self) -> String {
        self.name.clone()
    }
    pub fn return_team_role(&self) -> TeamRoles {
        self.role
    }
}

pub fn load_list(folder_name: &str) -> Vec<String> {
    let mut tmp_list = Vec::new();
    let file_path = "../teams".to_owned() + folder_name + "player_list.txt";

    if let Ok(file) = File::open(file_path) {
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        
        reader.read_line(&mut buf).unwrap();

        tmp_list.push(buf);
    }
    assert!(tmp_list.is_empty());
    tmp_list
}
