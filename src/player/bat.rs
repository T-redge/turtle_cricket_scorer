pub struct Batter {
    balls: u16,
    runs: u16,
}
impl Batter {
    pub fn new() -> Self {
        Self { balls: 0, runs: 0 }
    }
    pub fn return_balls_faced(&self) -> u16 {
        self.balls
    }
    pub fn return_runs_scored(&self) -> u16 {
        self.runs
    }
    pub fn runs_scored(&mut self, runs: u16) {
        self.balls += 1;
        self.runs += runs;
    }
}
