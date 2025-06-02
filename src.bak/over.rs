#[derive(Clone, Copy, Debug)]
pub enum BallEvent {
    EventWaiting,
    EventDot,
    EventRuns(RunsEvent),
    EventExtras(ExtrasEvent),
    EventWicket(WicketEvents),
}
#[derive(Clone, Copy, Debug)]
pub enum RunsEvent {
    OneScored(u8),
    TwoScored(u8),
    ThreeScored(u8),
    FourScored(u8),
    SixScored(u8),
}
#[derive(Clone, Copy, Debug)]
pub enum ExtrasEvent {
    WideBowled(u8),
    NoBallBowled(u8),
    ByesBowled(u8),
    LegbyesBowled(u8),
}
#[derive(Clone, Copy, Debug)]
pub enum WicketEvents {
    OutBowled,
    OutCaught,
    OutLBW,
    OutStumped,
    OutRunout,
}
#[derive(Clone, Debug)]
pub struct Over {
    pub ball_event: BallEvent,
    ball_bowled_flag: bool,
    balls_bowled: u8,
    runs_off_over: u8,
    wickets_taken: u8,
    ball_history: Vec<String>,
}
impl Over {
    pub fn new() -> Self {
        Over {
            ball_event: BallEvent::EventWaiting,
            ball_bowled_flag: false,
            balls_bowled: 0,
            runs_off_over: 0,
            wickets_taken: 0,
            ball_history: vec![],
        }
    }
    pub fn ball_bowled(&mut self) {
        self.add_ball();
        self.match_ball_event();
        self.change_ball_flag(false);
    }
    fn match_ball_event(&mut self) {
        match &self.ball_event {
            BallEvent::EventWaiting => {}
            BallEvent::EventDot => {
                println!("Dot ball!");
                self.ball_history.push(". ".to_string());
            }
            BallEvent::EventRuns(runs) => match runs {
                RunsEvent::OneScored(runs) => {
                    println!("One run scored!");
                    let mut runs_scored = String::from(runs.to_string());
                    runs_scored.push_str(" ");
                    self.ball_history.push(runs_scored);
                    self.runs_off_over += runs;
                }
                RunsEvent::TwoScored(runs) => {
                    println!("Two runs scored!");
                    let mut runs_scored = String::from(runs.to_string());
                    runs_scored.push_str(" ");
                    self.ball_history.push(runs_scored);
                    self.runs_off_over += runs;
                }
                RunsEvent::ThreeScored(runs) => {
                    println!("Three runs scored!");
                    let mut runs_scored = String::from(runs.to_string());
                    runs_scored.push_str(" ");
                    self.ball_history.push(runs_scored);
                    self.runs_off_over += runs;
                }
                RunsEvent::FourScored(runs) => {
                    println!("Four runs scored!");
                    let mut runs_scored = String::from(runs.to_string());
                    runs_scored.push_str(" ");
                    self.ball_history.push(runs_scored);
                    self.runs_off_over += runs;
                }
                RunsEvent::SixScored(runs) => {
                    println!("Six runs scored!");
                    let mut runs_scored = String::from(runs.to_string());
                    runs_scored.push_str(" ");
                    self.ball_history.push(runs_scored);
                    self.runs_off_over += runs;
                }
            },
            BallEvent::EventExtras(extras) => match extras {
                ExtrasEvent::WideBowled(runs) => {
                    println!("Wide bowled!");
                    let mut wide = runs.to_string();
                    wide.push_str("wd ");
                    self.ball_history.push(wide);
                    self.balls_bowled -= 1;
                    self.runs_off_over += runs;
                }
                ExtrasEvent::NoBallBowled(runs) => {
                    println!("Noball bowled!");
                    let mut noball = runs.to_string();
                    noball.push_str("+nb ");
                    self.ball_history.push(noball);
                    self.balls_bowled -= 1;
                    self.runs_off_over += runs + 1;
                }
                ExtrasEvent::ByesBowled(runs) => {
                    println!("Bye bowled!");
                    let mut b = runs.to_string();
                    b.push_str("b ");
                    self.ball_history.push(b);
                    self.runs_off_over += runs;
                }
                ExtrasEvent::LegbyesBowled(runs) => {
                    println!("Legbye bowled!");
                    let mut lb = runs.to_string();
                    lb.push_str("lb ");
                    self.ball_history.push(lb);
                    self.runs_off_over += runs;
                }
            },
            BallEvent::EventWicket(wicket) => {
                self.wickets_taken += 1;
                match wicket {
                    WicketEvents::OutBowled => {
                        println!("Batter out bowled!");
                        self.ball_history.push("wkt ".to_string());
                    }
                    WicketEvents::OutCaught => {
                        println!("Batter out caught!");
                        self.ball_history.push("wkt ".to_string());
                    }
                    WicketEvents::OutLBW => {
                        println!("Batter out lbw!");
                        self.ball_history.push("wkt ".to_string());
                    }
                    WicketEvents::OutStumped => {
                        println!("Batter out stumped!");
                        self.ball_history.push("wkt ".to_string());
                    }
                    WicketEvents::OutRunout => {
                        println!("Batter out run out!");
                        self.ball_history.push("wkt ".to_string());
                    }
                }
            }
        }
    }
    fn add_ball(&mut self) {
        self.balls_bowled += 1;
    }
    pub fn return_ball_count(&self) -> u8 {
        self.balls_bowled
    }
    pub fn return_runs_conceded(&self) -> u8 {
        self.runs_off_over
    }
    pub fn return_wickets_taken(&self) -> u8 {
        self.wickets_taken
    }
    pub fn change_ball_flag(&mut self, flag: bool) {
        self.ball_bowled_flag = flag;
    }
    pub fn return_ball_flag(&self) -> bool {
        self.ball_bowled_flag
    }
    pub fn return_ball_history(&self) -> String {
        let mut tmp = String::from("| ");
        for x in &self.ball_history {
            tmp.push_str(x);
        }
        tmp.push_str("|");
        tmp
    }
}
