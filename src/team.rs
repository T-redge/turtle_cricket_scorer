use crate::player::{BattingStatus, BowlingStatus, Player, bat::BatterStrike};
use std::{fs, io::*};

pub struct Team {
    name: &'static str,
    pub players: Vec<Player>,
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
    pub fn return_current_bowler(&self) -> usize {
        let mut player_num = 0;
        while player_num < 11 {
            if self.players[player_num].return_bowler_status() == BowlingStatus::Bowling {
                break;
            }
            player_num += 1;
        }
        player_num
    }
    pub fn return_last_over_bowler(&self) -> usize {
        let mut player_num = 0;
        while player_num < 11 {
            if self.players[player_num].return_bowler_status() == BowlingStatus::BowledLastOver {
                break;
            }
            player_num += 1;
        }
        player_num
    }
    pub fn return_batting_pair(&self) -> (usize, usize) {
        let mut player_num = 0;
        let (mut b_1, mut b_2) = (0, 0);
        while player_num < 11 {
            if self.players[player_num].return_batting_status() == BattingStatus::Batting {
                b_1 = player_num;
                player_num = 0;
                break;
            }
            player_num += 1;
        }
        while player_num < 11 {
            if self.players[player_num].return_name() == self.players[b_1].return_name() {
                player_num += 1;
            }
            if self.players[player_num].return_batting_status() == BattingStatus::Batting {
                b_2 = player_num;
                break;
            }
            player_num += 1;
        }
        let batters = (b_1, b_2);
        batters
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
    pub fn bowler_wicket_taken(&mut self, player: usize) {
        self.players[player].bowler_taken_wicket();
    }
    pub fn change_batters_strike(&mut self) {
        let batters = self.return_batting_pair();
        let bat_1 = self.players[batters.0].return_batter_strike();
        match bat_1 {
            BatterStrike::OnStrike => {
                self.players[batters.0].set_batter_strike(BatterStrike::OffStrike);
                self.players[batters.1].set_batter_strike(BatterStrike::OnStrike);
            }
            BatterStrike::OffStrike => {
                self.players[batters.0].set_batter_strike(BatterStrike::OnStrike);
                self.players[batters.1].set_batter_strike(BatterStrike::OffStrike);
            }
        }
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
