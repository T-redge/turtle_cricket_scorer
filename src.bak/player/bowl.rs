#[derive(Debug, Clone)]
pub struct Bowl {
    balls_bowled: u8,
    overs_bowled: u8,
}
impl Bowl {
    pub fn new() -> Self {
        Bowl {
            balls_bowled: 0,
            overs_bowled: 0,
        }
    }
    pub fn add_ball_bowled(&mut self) {
        self.balls_bowled += 1;
    }
    pub fn add_over_bowled(&mut self) {
        self.overs_bowled += 1;
    }
}
