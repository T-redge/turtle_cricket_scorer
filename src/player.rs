pub mod bat;
use crate::player::{bat::*, bowl::Bowler};
pub mod bowl;
pub mod field;
#[derive(Clone, Copy, PartialEq)]
pub enum BattingStatus {
    Waiting,
    Batting,
    Out,
}
#[derive(Clone, Copy, PartialEq)]
pub enum BowlingStatus {
    Waiting,
    Bowling,
    BowledLastOver,
}
pub struct Player {
    name: String,

    batter: Batter,
    batting_status: BattingStatus,

    bowler: Bowler,
    bowling_status: BowlingStatus,
    bowled: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,

            batter: Batter::new(),
            batting_status: BattingStatus::Waiting,

            bowler: Bowler::new(),
            bowling_status: BowlingStatus::Waiting,
            bowled: false,
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
    pub fn return_batting_status(&self) -> BattingStatus {
        self.batting_status
    }
    pub fn return_batter_strike(&self) -> BatterStrike {
        self.batter.return_batter_strike()
    }
    pub fn return_bowler_status(&self) -> BowlingStatus {
        self.bowling_status
    }
    pub fn return_scoreboard_profile(&self) -> (String, String, String, String) {
        let mut profile = (String::new(), String::new(), String::new(), String::new());
        profile.0 = self.return_name().to_string();
        if self.batting_status == BattingStatus::Out {
            profile.1 = self.batter.return_dismissal_type().to_string();
        } else if self.batting_status == BattingStatus::Batting {
            profile.1 = "Not Out".to_string();
        } else {
            profile.1 = "dnb".to_string();
        }
        profile.2 = self.batter.return_dismissed_by().to_string();
        profile.3 = self.return_batter_scores();
        profile
    }
    pub fn return_if_bowled(&self) -> bool {
        self.bowled
    }
    pub fn set_batter_strike(&mut self, strike: BatterStrike) {
        self.batter.set_batter_strike(strike);
    }
    pub fn set_batter_status(&mut self, status: BattingStatus) {
        self.batting_status = status;
    }
    pub fn set_bowler_status(&mut self, status: BowlingStatus) {
        self.bowling_status = status;
    }
    pub fn set_bowler_bool(&mut self, bool: bool) {
        self.bowled = bool;
    }
    pub fn set_batter_dismissal(&mut self, dismissal: &str) {
        self.batter.set_batter_dismissal(dismissal);
    }
    pub fn set_batter_dismissed_by(&mut self, bowler: &str) {
        self.batter.set_batter_dismissed_by(bowler);
    }
    pub fn batter_scored_runs(&mut self, runs: u16) {
        self.batter.runs_scored(runs);
    }
    pub fn batter_ball_faced(&mut self) {
        self.batter.ball_faced();
    }
    pub fn bowler_taken_wicket(&mut self) {
        self.bowler.wicket_taken();
    }
    pub fn bowler_ball_completed(&mut self) {
        self.bowler.ball_completed();
    }
    pub fn bowler_conceded_runs(&mut self, runs: u16) {
        self.bowler.runs_conceded(runs as u8);
    }
    pub fn bowler_over_completed(&mut self) {
        self.bowler.over_completed();
    }
}
