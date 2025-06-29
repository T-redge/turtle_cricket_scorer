#[derive(Copy, Clone, PartialEq)]
pub enum BatterStrike {
    OnStrike,
    OffStrike,
}
pub struct Batter {
    balls: u16,
    runs: u16,

    batter_strike: BatterStrike,

    dismissal_type: String,
    dismissed_by: String,
}
impl Batter {
    pub fn new() -> Self {
        Self {
            balls: 0,
            runs: 0,

            batter_strike: BatterStrike::OffStrike,

            dismissal_type: String::new(),
            dismissed_by: String::new(),
        }
    }
    pub fn return_balls_faced(&self) -> u16 {
        self.balls
    }
    pub fn return_runs_scored(&self) -> u16 {
        self.runs
    }
    pub fn return_batter_strike(&self) -> BatterStrike {
        self.batter_strike
    }
    pub fn return_dismissal_type(&self) -> &str {
        &self.dismissal_type
    }
    pub fn return_dismissed_by(&self) -> &str {
        &self.dismissed_by
    }
    pub fn ball_faced(&mut self) {
        self.balls += 1;
    }
    pub fn runs_scored(&mut self, runs: u16) {
        self.runs += runs;
    }
    pub fn set_batter_strike(&mut self, strike: BatterStrike) {
        self.batter_strike = strike;
    }
    pub fn set_batter_dismissal(&mut self, dismissal: &str) {
        self.dismissal_type = dismissal.to_string();
    }
    pub fn set_batter_dismissed_by(&mut self, bowler: &str) {
        self.dismissed_by = bowler.to_string();
    }
}
