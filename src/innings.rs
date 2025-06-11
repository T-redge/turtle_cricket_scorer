use crate::team::Team;

pub struct Innings {
    batting_team: Team,
    bowling_team: Team,

    inning_length: u8,
    overs_bowled: u8,
    balls_bowled: u8,
}

impl Innings {
    pub fn new(batting_team: &'static str, bowling_team: &'static str) -> Self {
        Self {
            batting_team: Team::new(batting_team),
            bowling_team: Team::new(bowling_team),

            inning_length: 5,
            overs_bowled: 0,
            balls_bowled: 0,
        }
    }
    pub fn return_team_names(&self) -> (String,String) {
        (
            self.batting_team.return_team_name(),
            self.bowling_team.return_team_name()
        )
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
    pub fn ball_bowled(&mut self) {
        self.balls_bowled += 1;
    }
    pub fn over_bowled(&mut self) {
        self.overs_bowled += 1;
        self.balls_bowled = 0;
    }
    pub fn check_innings_finished(&self) -> bool {
        if self.overs_bowled == self.inning_length {
            return true;
        } else {
            return false;
        }
    }
    pub fn check_over_finished(&self) -> bool {
        if self.balls_bowled == 6 {
            return true;
        } else {
            return false;
        }

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