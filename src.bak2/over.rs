#[derive(Debug)]
pub struct Over {
    balls_bowled: u8,
    runs_scored: u8,
    wickets_taken: u8,
}
impl Over {
    pub fn new() -> Self {
        Over {
            balls_bowled: 0,
            runs_scored: 0,
            wickets_taken: 0,
        }
    }
    pub fn add_ball_bowled(&mut self) {
        self.balls_bowled += 1;
    }
    pub fn add_run_scored(&mut self, amt: u8) {
        self.runs_scored += amt;
    }
    pub fn add_wicket_taken(&mut self) {
        self.wickets_taken += 1;
    }
    pub fn return_ball_bowled(&self) -> u8 {
        self.balls_bowled
    }
    pub fn return_runs_scored(&self) -> u8 {
        self.runs_scored
    }
    pub fn return_wickets_taken(&self) -> u8 {
        self.wickets_taken
    }
}
