#[derive(Debug, Clone)]
pub struct Bat {
    runs: u16,
    balls_faced: u8,
}
impl Bat {
    pub fn new() -> Self {
        Bat {
            runs: 0,
            balls_faced: 0,
        }
    }
    pub fn add_runs(&mut self, amt: u16) {
        self.runs += amt;
    }
    pub fn add_ball_faced(&mut self) {
        self.balls_faced += 1;
    }
}
