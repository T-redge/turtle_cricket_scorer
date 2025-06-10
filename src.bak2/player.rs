pub mod bat;
pub mod bowl;
use crate::player::{bat::*, bowl::*};
#[derive(Debug, Copy, Clone)]
pub enum PlayerBatStatus {
    WaitingToBat,
    InTheMiddle,
    Dismissed,
}
#[derive(Debug, Copy, Clone)]

pub enum PlayerBowlStatus {
    WaitingToBowl,
    IsBowling,
    BowledLastOver,
}
#[derive(Debug, Copy, Clone)]
pub enum PlayerStrike {
    OnStrike,
    OffStrike,
}
#[derive(Debug, Clone)]

pub struct Player {
    player_name: String,
    bat_profile: Bat,
    bowl_profile: Bowl,
    player_bat_status: PlayerBatStatus,
    player_strike: PlayerStrike,
    player_bowl_status: PlayerBowlStatus,
}
impl Player {
    pub fn new(player_name: &str) -> Self {
        Player {
            player_name: player_name.to_owned(),
            bat_profile: Bat::new(),
            bowl_profile: Bowl::new(),
            player_bat_status: PlayerBatStatus::WaitingToBat,
            player_strike: PlayerStrike::OffStrike,
            player_bowl_status: PlayerBowlStatus::WaitingToBowl,
        }
    }
    pub fn set_player_bat_status(&mut self, p_status: PlayerBatStatus) {
        self.player_bat_status = p_status;
    }
    pub fn set_player_bowl_status(&mut self, p_status: PlayerBowlStatus) {
        self.player_bowl_status = p_status;
    }
    pub fn set_player_batter_strike(&mut self, strike: PlayerStrike) {
        self.player_strike = strike;
    }
    pub fn return_batter_strike_status(&self) -> PlayerStrike {
        self.player_strike
    }
    pub fn return_player_bat_status(&self) -> PlayerBatStatus {
        self.player_bat_status
    }
    pub fn return_player_bowl_status(&self) -> PlayerBowlStatus {
        self.player_bowl_status
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
    pub fn player_bowled_over(&mut self) {
        self.bowl_profile.add_over_bowled();
        self.bowl_profile.reset_balls_bowled();
    }
}
