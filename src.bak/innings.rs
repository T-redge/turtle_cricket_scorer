use crate::{extras::*, over::*, player::bowl::Bowl, team::*};
use eframe::egui::{Color32, Response, RichText, Ui, Widget};
#[derive(Clone, Debug)]
pub struct Innings {
    pub current_over: Over,
    over_count: usize,
    pub over_history: Vec<Over>,
    bat_score: u16,
    bowl_wkt: u8,
    pub extras: Extras,
    pub home_team: Team,
    pub away_team: Team,
    pub current_bowler: Bowl,
}
impl Widget for Innings {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut overs = String::from("\t\t    Team:\t");
        overs.push_str(&self.bat_score.to_string());
        overs.push_str("/");
        overs.push_str(&self.bowl_wkt.to_string());
        overs.push_str("\tOvers:\t");
        overs.push_str(&self.over_count.to_string());
        overs.push_str(".");
        overs.push_str(&self.current_over.return_ball_count().to_string());
        overs.push_str("\n");
        overs.push_str(&self.extras.print_extras());

        let text = RichText::new(overs)
            .size(20.0)
            .monospace()
            .color(Color32::WHITE);
        let response = ui.label(text);
        response
    }
}
impl Innings {
    pub fn new() -> Self {
        Innings {
            current_over: Over::new(),
            over_count: 0,
            over_history: vec![],
            bat_score: 0,
            bowl_wkt: 0,
            extras: Extras::new(),
            home_team: Team::new("teamlists/edgewater.txt"),
            away_team: Team::new("teamlists/kingsway.txt"),
            current_bowler: Bowl::new(),
        }
    }
    pub fn end_over(&mut self) -> bool {
        if self.current_over.return_ball_count() == 6 {
            true
        } else {
            false
        }
    }
    pub fn commit_to_overhistory(&mut self) {
        self.over_history.push(self.current_over.clone());
    }
    pub fn return_over_history(&self, over_number: usize) -> Over {
        self.over_history[over_number].clone()
    }
    pub fn return_over_count(&self) -> usize {
        self.over_count
    }
    pub fn add_over_count(&mut self) {
        self.over_count += 1;
    }
    pub fn add_score(&mut self, amount: u16) {
        self.bat_score += amount;
    }
    pub fn return_home_team(&self) -> String {
        let mut ht = String::new();
        ht.push_str("\n");
        for player in &self.home_team.players {
            ht.push_str(&player.name);
            ht.push_str("\n");
        }
        ht
    }
    pub fn return_away_team(&self) -> String {
        let mut at = String::new();
        at.push_str("\n");
        for player in &self.away_team.players {
            at.push_str(&player.name);
            at.push_str("\n");
        }
        at
    }
}
