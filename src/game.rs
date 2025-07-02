use crate::team::*;
pub struct Game {
    team_1: Team,
    team_2: Team,
}

impl Game {
    pub fn new() -> Self {
        let t1_name = String::from("Edgewater");
        let t2_name = String::from("Kingsway");
        Self {
            team_1: Team::new(&t1_name),
            team_2: Team::new(&t2_name),
        }
    }
    pub fn return_team_names(&self) -> (String,String) {
        let names = (self.team_1.return_team_name().to_owned(), self.team_2.return_team_name().to_owned());
        names
    }
}