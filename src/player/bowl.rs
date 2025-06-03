#[derive(Debug,Copy,Clone)]
pub struct Bowl {
    player_overs: u8,
    player_balls: u8,
    player_maidens: u8,
    player_runs: u8,
    player_wickets: u8,
}
impl Bowl {
    pub fn new() -> Self {
        Bowl {
            player_overs: 0,
            player_balls: 0,
            player_maidens: 0,
            player_runs: 0,
            player_wickets: 0,
        }
    }
//Change fields
    pub fn add_ball_bowled(&mut self) {
        self.player_balls += 1;
    }
//Return Functions
    pub fn return_bowling_figures(&self) -> String {
        let mut bowl_figure = self.player_overs.to_string();
        bowl_figure.push_str(".");
        bowl_figure.push_str(&self.player_balls.to_string());
        bowl_figure.push_str("-");
        bowl_figure.push_str(&self.player_maidens.to_string());
        bowl_figure.push_str("-");
        bowl_figure.push_str(&self.player_runs.to_string());
        bowl_figure.push_str("-");
        bowl_figure.push_str(&self.player_wickets.to_string());
        bowl_figure
    }
}
