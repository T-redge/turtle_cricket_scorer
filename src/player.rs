pub mod bat;
pub mod bowl;
use crate::player::{bat::*, bowl::*};
pub struct Player {
    player_name: &'static str,
    bat_profile: Bat,
    bowl_profile: Bowl,
}
impl Player {
    pub fn new(player_name: &'static str) -> Self {
        Player {
            player_name,
            bat_profile: Bat::new(),
            bowl_profile: Bowl::new(),
        }
    }
    pub fn add_ball_faced(&mut self) {
        self.bat_profile.add_ball_faced();
    }
    pub fn add_run_scored(&mut self, amt: u16) {
        self.bat_profile.add_run_scored(amt);
    }
    pub fn return_player_name(&self) -> &'static str {
        self.player_name
    }
    pub fn return_player_bat_profile(&self) -> String {
        self.bat_profile.return_batter_score()
    }
    pub fn return_player_bowl_profile(&self) -> String {
        self.bowl_profile.return_bowling_figures()
    }
}
