use crate::{extras::Extras, player::Player};
pub struct Team {
    team_name: &'static str,
    team_runs: u16,
    team_wickets: u8,
    team_overs: u8,
    team_balls: u8,
    team_extras: Extras,
    player_1: Player,
    player_2: Player,
}
impl Team {
    pub fn new(team_name: &'static str) -> Self {
        Team {
            team_name,
            team_runs: 0,
            team_wickets: 0,
            team_overs: 0,
            team_balls: 0,
            team_extras: Extras::new(),
            player_1: Player::new("Alex Tredgett"),
            player_2: Player::new("Callum Lloyd-Watters"),
        }
    }
    //Changing fields functions
    pub fn add_team_runs(&mut self, amt: u16) {
        self.team_runs += amt;
    }
    pub fn add_team_wickets(&mut self) {
        self.team_wickets += 1;
    }
    //Return functions
    pub fn return_team_score(&self) -> (String, String) {
        let mut team_total = self.team_name.to_string();
        team_total.push_str("\t");
        team_total.push_str(&self.team_runs.to_string());
        team_total.push_str("/");
        team_total.push_str(&self.team_wickets.to_string());
        let mut over_total = String::from("Over:\t");
        over_total.push_str(&self.team_overs.to_string());
        over_total.push_str(".");
        over_total.push_str(&self.team_balls.to_string());
        (team_total, over_total)
    }
    pub fn return_player_names(&self) -> (&'static str, &'static str) {
        let p1 = self.player_1.return_player_name();
        let p2 = self.player_2.return_player_name();
        (p1, p2)
    }
    pub fn return_players_bat_profile(&self) -> (String, String) {
        let p1_bat = self.player_1.return_player_bat_profile();
        let p2_bat = self.player_2.return_player_bat_profile();
        (p1_bat, p2_bat)
    }
    pub fn return_players_bowl_profile(&self) -> (String, String) {
        let p1_bowl = self.player_1.return_player_bowl_profile();
        let p2_bowl = self.player_2.return_player_bowl_profile();
        (p1_bowl, p2_bowl)
    }
    pub fn return_extras(&self) -> String {
        let extras = self.team_extras.return_extras();
        extras
    }
}
