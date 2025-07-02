pub struct Innings {
    total_overs: u8,
    overs_completed: u8,
}

impl Innings {
    pub fn new(total_overs: u8) -> Innings {
        Innings {
            total_overs: total_overs,
            overs_completed: 0,
        }  
    }
    pub fn return_innings_finished(&self) -> bool {
        let complete = self.total_overs == self.overs_completed;
        if complete {
            return true
        } else {
            return false
        }
    }
}