pub struct Innings {
    inning_length: u8,
    overs_bowled: u8,
    balls_bowled: u8,
}

impl Innings {
    pub fn new() -> Self {
        Self {
            inning_length: 20,
            overs_bowled: 0,
            balls_bowled: 0,
        }
    }
    pub fn return_overs_label(&self) -> String {
        let mut over_label = "Overs:\t".to_string();
        over_label.push_str(&self.return_overs_bowled().to_string());
        over_label.push_str(".");
        over_label.push_str(&self.return_balls_bowled().to_string());
        over_label.push_str("(");
        over_label.push_str(&self.return_inning_length().to_string());
        over_label.push_str(")");
        over_label
    }
    fn return_inning_length(&self) -> u8 {
        self.inning_length
    }
    fn return_overs_bowled(&self) -> u8 {
        self.overs_bowled
    }
    fn return_balls_bowled(&self) -> u8 {
        self.balls_bowled
    }
}
