pub struct Bat {
    player_runs: u16,
    player_balls: u16,
}
impl Bat {
    pub fn new() -> Self {
        Bat {
            player_runs: 0,
            player_balls: 0,
        }
    }
    pub fn add_ball_faced(&mut self) {
        self.player_balls += 1;
    }
    pub fn add_run_scored(&mut self, amt: u16) {
        self.player_runs += amt;
    }
    pub fn return_batter_score(&self) -> String {
        let mut bat_score = self.player_runs.to_string();
        bat_score.push_str("(");
        bat_score.push_str(&self.player_balls.to_string());
        bat_score.push_str(")");
        bat_score
    }
}
