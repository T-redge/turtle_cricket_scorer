use crate::{extras::Extras, player::*};
use std::{
    fs,
    io::{BufRead, BufReader},
};
#[derive(Debug, Clone, Copy)]
pub enum TeamRole {
    BattingTeam,
    BowlingTeam,
}
#[derive(Debug)]
pub struct Team {
    team_name: String,
    team_runs: u16,
    team_wickets: u8,
    team_overs: u8,
    team_balls: u8,
    team_role: TeamRole,
    team_extras: Extras,
    team_player: Box<Vec<Player>>,
}
impl Team {
    //Changing fields
    pub fn new(team_name: &str, team_role: TeamRole) -> Self {
        let (name, player_list) = load_team_list(team_name);
        Team {
            team_name: name,
            team_runs: 0,
            team_wickets: 0,
            team_overs: 0,
            team_balls: 0,
            team_role,
            team_extras: Extras::new(),
            team_player: Box::new(player_list),
        }
    }
    pub fn set_batter_strike(&mut self, player: usize, strike: PlayerStrike) {
        self.team_player[player].set_player_batter_strike(strike);
    }
    pub fn set_player_bat_status(&mut self, player: usize, p_status: PlayerBatStatus) {
        self.team_player[player].set_player_bat_status(p_status);
    }
    pub fn set_player_bowl_status(&mut self, player: usize, p_status: PlayerBowlStatus) {
        self.team_player[player].set_player_bowl_status(p_status);
    }
    pub fn add_team_runs(&mut self, amt: u16) {
        self.team_runs += amt;
    }
    pub fn add_team_wickets(&mut self) {
        self.team_wickets += 1;
    }
    pub fn add_bat_ball_faced(&mut self) {
        let (b1, b2) = self.return_player_at_middle_usize();
        match self.team_player[b1].return_batter_strike_status() {
            PlayerStrike::OnStrike => {
                self.team_player[b1].add_ball_faced();
            }
            PlayerStrike::OffStrike => {
                self.team_player[b2].add_ball_faced();
            }
        }
    }
    pub fn add_over_bowled(&mut self) {
        self.team_overs += 1;
    }
    pub fn add_bowler_ball_bowled(&mut self) {
        let mut bowler = 0;
        loop {
            match self.team_player[bowler].return_player_bowl_status() {
                PlayerBowlStatus::IsBowling => {
                    break;
                }
                _ => {}
            }
            bowler += 1;
        }
        self.team_player[bowler].add_ball_bowled();
        self.team_balls += 1;
    }
    pub fn reset_team_balls_bowled(&mut self) {
        let bowler = self.return_player_bowling();
        self.team_balls = 0;
        self.team_player[bowler].player_bowled_over();
    }
    //Return fields
    pub fn return_over_number(&self) -> u8 {
        self.team_overs
    }
    pub fn return_team_balls_bowled(&self) -> u8 {
        self.team_balls
    }
    pub fn return_player_bat_status(&self, player: usize) -> PlayerBatStatus {
        self.team_player[player].return_player_bat_status()
    }
    pub fn return_player_bowl_status(&self, player: usize) -> PlayerBowlStatus {
        self.team_player[player].return_player_bowl_status()
    }
    pub fn return_player_at_middle(&self) -> (Player, Player) {
        let mut b1_count = 0;
        let mut b2_count = 0;
        'b1: loop {
            match self.team_player[b1_count].return_player_bat_status() {
                PlayerBatStatus::InTheMiddle => break 'b1,
                _ => {}
            }
            b1_count += 1;
        }
        'b2: loop {
            if b1_count == b2_count {
                b2_count += 1;
            }
            match self.team_player[b2_count].return_player_bat_status() {
                PlayerBatStatus::InTheMiddle => break 'b2,
                _ => {}
            }
            b2_count += 1;
        }
        let b1 = self.team_player[b1_count].clone();
        let b2 = self.team_player[b2_count].clone();

        (b1, b2)
    }
    pub fn return_player_at_middle_usize(&self) -> (usize, usize) {
        let mut b1_count = 0;
        let mut b2_count = 0;
        'b1: loop {
            match self.team_player[b1_count].return_player_bat_status() {
                PlayerBatStatus::InTheMiddle => break 'b1,
                _ => {}
            }
            b1_count += 1;
        }
        'b2: loop {
            if b1_count == b2_count {
                b2_count += 1;
            }
            match self.team_player[b2_count].return_player_bat_status() {
                PlayerBatStatus::InTheMiddle => break 'b2,
                _ => {}
            }
            b2_count += 1;
        }

        (b1_count, b2_count)
    }
    pub fn return_player_bowling(&self) -> usize {
        let mut bowler_count = 0;
        loop {
            match self.team_player[bowler_count].return_player_bowl_status() {
                PlayerBowlStatus::IsBowling => break,
                _ => {}
            }
            bowler_count += 1;
        }
        bowler_count
    }
    pub fn return_player_bowled_last_over(&self) -> usize {
        let mut bowler_count = 0;
        loop {
            match self.team_player[bowler_count].return_player_bowl_status() {
                PlayerBowlStatus::BowledLastOver => break,
                _ => {}
            }
            bowler_count += 1;
        }
        bowler_count
    }
    pub fn return_team_score(&self) -> String {
        let mut team_total = self.team_name.to_string();
        team_total.push_str("\t");
        team_total.push_str(&self.team_runs.to_string());
        team_total.push_str("/");
        team_total.push_str(&self.team_wickets.to_string());
        
        team_total
    }
    pub fn return_over_total(&self) -> String {
        let mut over = String::from("Overs:\t");
        over.push_str(&self.team_overs.to_string());
        over.push_str(".");
        over.push_str(&self.team_balls.to_string());
        over
    }
    pub fn return_player_names(&self, player: usize) -> &str {
        self.team_player[player].return_player_name()
    }
    pub fn return_players_bat_profile(&self) -> (String, String) {
        let p1_bat = self.team_player[0].return_player_bat_profile();
        let p2_bat = self.team_player[0].return_player_bat_profile();
        (p1_bat, p2_bat)
    }
    pub fn return_players_bowl_profile(&self, player_num: usize) -> String {
        let p1_bowl = self.team_player[player_num].return_player_bowl_profile();
        p1_bowl
    }
    pub fn return_extras(&self) -> String {
        let extras = self.team_extras.return_extras();
        extras
    }
    pub fn return_team_role(&self) -> TeamRole {
        self.team_role
    }
    pub fn return_team_name(&self) -> &str {
        &self.team_name
    }
}

pub fn load_team_list(team_name: &str) -> (String, Vec<Player>) {
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
    let tm_name = tmp[0].clone();
    for x in 1..=11 {
        tmp_plyr.push(Player::new(&tmp[x].to_owned()));
    }
    (tm_name, tmp_plyr)
}
