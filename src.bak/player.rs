pub mod bat;
pub mod bowl;
use crate::player::{bat::*, bowl::*};
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub bat_profile: Bat,
    pub bowl_profile: Bowl,
}
impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            bat_profile: Bat::new(),
            bowl_profile: Bowl::new(),
        }
    }
}
