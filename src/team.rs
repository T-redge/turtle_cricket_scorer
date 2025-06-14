use crate::player::Player;
use std::{fs, io::*};

pub struct Team {
    name: &'static str,
    players: Vec<Player>,
}
impl Team {
    pub fn new(name: &'static str) -> Self {
        let players = load_team_file(name);
        Self { name, players }
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
    pub fn return_player_batter_score(&self, player_number: usize) -> (String, String) {
        let batter_name = self.players[player_number].return_name().to_string();
        let batter_scores = self.players[player_number].return_batter_scores();
        (batter_name, batter_scores)
    }
    pub fn return_player_bowler_score(&self, player_number: usize) -> (String, String) {
        let bowler_name = self.players[player_number].return_name().to_string();
        let bowler_scores = self.players[player_number]
            .return_bowler_scores()
            .to_string();
        (bowler_name, bowler_scores)
    }
    pub fn batter_scored_runs(&mut self, player: usize, runs: u16) {
        self.players[player].batter_scored_runs(runs);
    }
    pub fn batter_ball_faced(&mut self, player: usize) {
        self.players[player].batter_ball_faced();
    }
    pub fn bowler_ball_completed(&mut self, player: usize) {
        self.players[player].bowler_ball_completed();
    } 
    pub fn bowler_conceded_runs(&mut self, player: usize, runs: u16) {
        self.players[player].bowler_conceded_runs(runs);
    }
    pub fn bowler_over_completed(&mut self, player: usize) {
        self.players[player].bowler_over_completed();
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
