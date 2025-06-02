use std::{fs, io::*};

use crate::player::*;
#[derive(Debug, Clone)]
pub struct Team {
    pub team_name: String,
    pub players: Vec<Player>,
}
impl Team {
    pub fn new(file_path: &'static str) -> Self {
        let team_file = fs::File::open(file_path).unwrap();
        let mut team_list: Vec<String> = Vec::new();
        let buf = BufReader::new(team_file);

        for line in buf.lines() {
            team_list.push(line.unwrap());
        }
        let team_nm = team_list[0].clone();
        let team_players = team_list.split_off(1);
        let mut play_xi: Vec<Player> = Vec::new();
        for x in team_players {
            play_xi.push(Player::new(x.clone()));
        }

        Team {
            team_name: team_nm,
            players: play_xi,
        }
    }
}
