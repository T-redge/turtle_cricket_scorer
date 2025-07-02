use crate::team::*;

pub struct Innings {
    _batting_team: Team,
    _bowling_team: Team,

    total_overs: u8,
}

impl Innings {
    pub fn new() -> Self {
        Self {
            _batting_team: Team,
            _bowling_team: Team,
            total_overs: 0,
        }
    }
}