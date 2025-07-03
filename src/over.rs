pub struct Over {
    balls: u8,
}

impl Over {
    pub fn new() -> Self {
        Self { balls: 0 }
    }
    pub fn check_over_complete(&self) -> bool {
        if self.balls == 6 {
            return true;
        } else {
            return false;
        }
    }
    pub fn return_balls(&self) -> u8 {
        self.balls
    }
}
