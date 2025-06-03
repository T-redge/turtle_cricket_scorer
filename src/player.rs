pub mod bat;
pub mod bowl;
use crate::player::{bat::*, bowl::*};
#[derive(Debug,Copy,Clone)]
pub enum PlayerStatus {
    WaitingToBat,
    InTheMiddle,
    Dismissed,
}
#[derive(Debug,Copy,Clone)]

pub enum PlayerRole {
    IsBatting,
    IsBowling
}
#[derive(Debug,Copy,Clone)]
pub enum PlayerStrike {
    OnStrike,
    OffStrike,
}
#[derive(Debug,Clone)]

pub struct Player {
    player_name: String,
    bat_profile: Bat,
    bowl_profile: Bowl,
    player_status: PlayerStatus,
}
impl Player {
    pub fn new(player_name: &str) -> Self {
        Player {
            player_name: player_name.to_owned(),
            bat_profile: Bat::new(),
            bowl_profile: Bowl::new(),
            player_status: PlayerStatus::WaitingToBat,
        }
    }
    pub fn set_player_status(&mut self, p_status: PlayerStatus) {
        self.player_status = p_status;
    }
    pub fn return_player_status(&self) -> PlayerStatus {
        self.player_status
    }
    pub fn add_ball_faced(&mut self) {
        self.bat_profile.add_ball_faced();
    }
    pub fn add_ball_bowled(&mut self) {
        self.bowl_profile.add_ball_bowled();
    }
    pub fn add_run_scored(&mut self, amt: u16) {
        self.bat_profile.add_run_scored(amt);
    }
    pub fn return_player_name(&self) -> &str {
        &self.player_name
    }
    pub fn return_player_bat_profile(&self) -> String {
        self.bat_profile.return_batter_score()
    }
    pub fn return_player_bowl_profile(&self) -> String {
        self.bowl_profile.return_bowling_figures()
    }
}
