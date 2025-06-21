#[derive(Copy, Clone)]
pub enum BatterStrike {
    OnStrike,
    OffStrike,
}
pub struct Batter {
    balls: u16,
    runs: u16,

    batter_strike: BatterStrike,
}
impl Batter {
    pub fn new() -> Self {
        Self { balls: 0, runs: 0, batter_strike: BatterStrike::OffStrike }
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
    pub fn ball_faced(&mut self) {
        self.balls += 1;
    }
    pub fn runs_scored(&mut self, runs: u16) {
        self.runs += runs;
    }
}
