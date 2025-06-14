mod bat;
use crate::player::{bat::*, bowl::Bowler};
mod bowl;
mod field;

pub struct Player {
    name: String,
    batter: Batter,
    bowler: Bowler,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            batter: Batter::new(),
            bowler: Bowler::new(),
        }
    }
    pub fn return_name(&self) -> &str {
        &self.name
    }
    pub fn return_batter_scores(&self) -> String {
        let mut bt_s = self.batter.return_runs_scored().to_string();
        bt_s.push_str("(");
        bt_s.push_str(&self.batter.return_balls_faced().to_string());
        bt_s.push_str(")");
        bt_s
    }
    pub fn return_bowler_scores(&self) -> String {
        let mut bl_s = self.bowler.overs_bowled().to_string();
        bl_s.push_str(".");
        bl_s.push_str(&self.bowler.balls_bowled().to_string());
        bl_s.push_str("-");
        bl_s.push_str(&self.bowler.maidens_bowled().to_string());
        bl_s.push_str("-");
        bl_s.push_str(&self.bowler.runs_bowled().to_string());
        bl_s.push_str("-");
        bl_s.push_str(&self.bowler.wickets_bowled().to_string());
        bl_s
    }
    pub fn player_scored_runs(&mut self, runs: u16) {
        self.batter.runs_scored(runs);
    }
    pub fn bowler_conceded_runs(&mut self, runs: u16) {
        self.bowler.runs_conceded(runs as u8);
    }
    pub fn bowler_over_completed(&mut self) {
        self.bowler.over_completed();
    }
}
