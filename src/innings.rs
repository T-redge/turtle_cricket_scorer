pub struct Innings {
    total_overs: u8,
    overs_completed: u8,

    total_runs: u16,
    total_wickets: u8,
}

impl Innings {
    pub fn new(total_overs: u8) -> Self {
        Self {
            total_overs: total_overs,
            overs_completed: 0,

            total_runs: 0,
            total_wickets: 0,
        }
    }
    pub fn return_innings_finished(&self) -> bool {
        let complete = self.total_overs == self.overs_completed;
        if complete { return true } else { return false }
    }
    pub fn return_total_wicket(&self) -> u8 {
        self.total_wickets
    }
    pub fn return_total_runs(&self) -> u16 {
        self.total_runs
    }
    pub fn return_overs_completed(&self) -> u8 {
        self.overs_completed
    }
}
