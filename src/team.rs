use crate::players::Player;
use std::{fs,io::*};

pub struct Team {
    name: &'static str,
    players: Vec<Player>
}
impl Team {
    pub fn new(name: &'static str) -> Self {
        let players = load_team_file(name);
        Self {
            name,
            players,
        }
    }
    pub fn return_team_name(&self) -> String {
        self.name.to_string()
    }
    pub fn return_team_players(&self) -> String {
        let mut team_list = String::new();
        team_list.push_str("\n");
        for x in &self.players {
            team_list.push_str(x.return_name());
            team_list.push_str("\n");
        }
        team_list
    }
}
fn load_team_file(team_name: &'static str) -> Vec<Player> {
    let mut tmp = Vec::new();
    let mut file_path = String::from("teamlists/");
    file_path.push_str(&team_name.to_ascii_lowercase());
    file_path.push_str(".txt");
    let t_list = fs::File::open(file_path).unwrap();
    let buf_reader = BufReader::new(t_list);
    for line in buf_reader.lines() {
        tmp.push(line.unwrap());
    }

    let mut tmp_plyr: Vec<Player> = Vec::new();
    for x in tmp.split_off(1) {
        tmp_plyr.push(Player::new(String::from(x)));
    }
    tmp_plyr
}