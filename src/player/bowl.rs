pub struct Bowler {
    overs: u8,
    balls: u8,
    maidens: u8,
    runs: u8,
    wickets: u8,
}
impl Bowler {
    pub fn new() -> Self {
        Self {
            overs: 0,
            balls: 0,
            maidens: 0,
            runs: 0,
            wickets: 0,
        }
    }
    pub fn overs_bowled(&self) -> u8 {
        self.overs
    }
    pub fn balls_bowled(&self) -> u8 {
        self.balls
    }
    pub fn maidens_bowled(&self) -> u8 {
        self.maidens
    }
    pub fn runs_bowled(&self) -> u8 {
        self.runs
    }
    pub fn wickets_bowled(&self) -> u8 {
        self.wickets
    }
    pub fn runs_conceded(&mut self, runs: u8) {
        self.balls += 1;
        self.runs += runs;
    }
    pub fn over_completed(&mut self) {
        self.overs += 1;
        self.balls = 0;
    }
}
