use crate::{player::bat::*, team::Team};

pub enum BallEvent {
    Waiting,
    DotBall,
    RunScored(u16),
    WicketBowler(&'static str),
    WicketTeam(&'static str),
    WideBowled(u8),
    NoballBowled(u8),
    ByeBowled(u8),
    LegbyeBowled(u8),
}

pub struct Innings {
    pub batting_team: Team,
    pub bowling_team: Team,

    inning_length: u8,
    overs_bowled: u8,
    balls_bowled: u8,

    over_total: u8,
    bat1_over_total: u8,
    bat2_over_total: u8,

    runs_total: u16,
    wicket_total: u8,

    wides_total: u8,
    noball_total: u8,
    byes_total: u8,
    legbyes_total: u8,

    ball_event: BallEvent,
}

impl Innings {
    pub fn new(batting_team: &'static str, bowling_team: &'static str) -> Self {
        Self {
            batting_team: Team::new(batting_team),
            bowling_team: Team::new(bowling_team),

            inning_length: 5,
            overs_bowled: 0,
            balls_bowled: 0,

            over_total: 0,
            bat1_over_total: 0,
            bat2_over_total: 0,

            runs_total: 0,
            wicket_total: 0,

            wides_total: 0,
            noball_total: 0,
            byes_total: 0,
            legbyes_total: 0,

            ball_event: BallEvent::Waiting,
        }
    }
    pub fn set_ball_event(&mut self, event: BallEvent) {
        self.ball_event = event;
    }
    pub fn match_ball_event(&mut self) {
        match self.ball_event {
            BallEvent::Waiting => {}
            BallEvent::DotBall => {
                self.ball_bowled();
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::RunScored(runs) => {
                self.ball_bowled();
                self.runs_scored(runs);
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::WicketBowler(_wicket) => {
                self.ball_bowled();
                self.bowler_wicket_taken();
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::WicketTeam(_wicket) => {
                self.ball_bowled();
                self.team_wicket_taken();
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::WideBowled(runs) => {
                self.bowler_wide_bowled(runs);
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::NoballBowled(runs) => {
                self.bowler_noball_bowled(runs);
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::ByeBowled(runs) => {
                self.ball_bowled();
                self.bowler_byes_bowled(runs);
                self.set_ball_event(BallEvent::Waiting);
            }
            BallEvent::LegbyeBowled(runs) => {
                self.ball_bowled();
                self.bowler_legbyes_bowled(runs);
                self.set_ball_event(BallEvent::Waiting);
            }
        }
    }
    pub fn bowler_wide_bowled(&mut self, runs: u8) {
        self.wides_total += runs;
        self.runs_total += runs as u16;
        self.over_total += runs;

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team.bowler_conceded_runs(bowler, runs.into());
    }
    pub fn bowler_noball_bowled(&mut self, runs: u8) {
        self.noball_total += 1;
        self.runs_total += runs as u16 + 1;
        self.over_total += runs + 1;

        let b1 = self.batting_team.return_batting_pair().0;
        let b2 = self.batting_team.return_batting_pair().1;

        if self.batting_team.players[b1].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_scored_runs(b1, runs as u16);
            self.bat1_over_total += runs as u8;
        }
        if self.batting_team.players[b2].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_scored_runs(b2, runs as u16);
            self.bat2_over_total += runs as u8;
        }

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team
            .bowler_conceded_runs(bowler, (runs + 1).into());
    }
    pub fn bowler_byes_bowled(&mut self, runs: u8) {
        self.byes_total += runs;
        self.runs_total += runs as u16;
        self.over_total += runs;
    }
    pub fn bowler_legbyes_bowled(&mut self, runs: u8) {
        self.legbyes_total += runs;
        self.runs_total += runs as u16;
        self.over_total += runs;
    }
    pub fn bowler_wicket_taken(&mut self) {
        self.wicket_total += 1;

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team.bowler_wicket_taken(bowler);
    }
    pub fn team_wicket_taken(&mut self) {
        self.wicket_total += 1;
    }
    pub fn runs_scored(&mut self, runs: u16) {
        self.runs_total += runs;
        self.over_total += runs as u8;
        let b1 = self.batting_team.return_batting_pair().0;
        let b2 = self.batting_team.return_batting_pair().1;

        if self.batting_team.players[b1].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_scored_runs(b1, runs);
            self.bat1_over_total += runs as u8;
        }
        if self.batting_team.players[b2].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_scored_runs(b2, runs);
            self.bat2_over_total += runs as u8;
        }

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team.bowler_conceded_runs(bowler, runs);
    }
    pub fn ball_bowled(&mut self) {
        self.balls_bowled += 1;

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team.bowler_ball_completed(bowler);

        let b1 = self.batting_team.return_batting_pair().0;
        let b2 = self.batting_team.return_batting_pair().1;

        if self.batting_team.players[b1].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_ball_faced(b1);
        }
        if self.batting_team.players[b2].return_batter_strike() == BatterStrike::OnStrike {
            self.batting_team.batter_ball_faced(b2);
        }
    }
    pub fn over_bowled(&mut self) {
        self.overs_bowled += 1;
        self.balls_bowled = 0;

        let bowler = self.bowling_team.return_current_bowler();
        self.bowling_team.bowler_over_completed(bowler);
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
    pub fn return_team_names(&self) -> (String, String) {
        (
            self.batting_team.return_team_name(),
            self.bowling_team.return_team_name(),
        )
    }
    pub fn return_team_lists(&self) -> (String, String) {
        let batting_team = self.batting_team.return_team_players();
        let bowling_team = self.bowling_team.return_team_players();

        (batting_team, bowling_team)
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
    pub fn return_team_score(&self) -> String {
        let mut team_score = self.return_team_total().to_string();
        team_score.push_str("/");
        team_score.push_str(&self.return_team_wickets().to_string());
        team_score
    }
    pub fn return_team_extras(&self) -> String {
        let mut extras = String::from("Extras ( Wd: ");
        extras.push_str(&self.return_wides().to_string());
        extras.push_str(" Nb: ");
        extras.push_str(&self.return_noballs().to_string());
        extras.push_str(" B: ");
        extras.push_str(&self.return_byes().to_string());
        extras.push_str(" Lb: ");
        extras.push_str(&self.return_legbyes().to_string());
        extras.push_str(" )\n");
        extras
    }
    pub fn return_over_totals(&self) -> String {
        let batters = self.batting_team.return_batting_pair();
        let mut over_total = String::from("Over total:\t");
        over_total.push_str(&self.return_over_total().to_string());
        over_total.push_str("\n\n");
        over_total.push_str(&self.batting_team.players[batters.0].return_name());
        over_total.push_str(":\t");
        over_total.push_str(&self.return_bat1_over_scores().to_string());
        over_total.push_str("\n");
        over_total.push_str(&self.batting_team.players[batters.1].return_name());
        over_total.push_str(":\t");
        over_total.push_str(&self.return_bat2_over_scores().to_string());

        over_total
    }
    fn return_bat1_over_scores(&self) -> u8 {
        self.bat1_over_total
    }
    fn return_bat2_over_scores(&self) -> u8 {
        self.bat2_over_total
    }
    fn return_over_total(&self) -> u8 {
        self.over_total
    }
    fn return_wides(&self) -> u8 {
        self.wides_total
    }
    fn return_noballs(&self) -> u8 {
        self.noball_total
    }
    fn return_byes(&self) -> u8 {
        self.byes_total
    }
    fn return_legbyes(&self) -> u8 {
        self.legbyes_total
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
    fn return_team_total(&self) -> u16 {
        self.runs_total
    }
    fn return_team_wickets(&self) -> u8 {
        self.wicket_total
    }
}
