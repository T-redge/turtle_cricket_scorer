use crate::{game::*, team_selector::*};
use eframe::egui::{self, Align2, Button, Color32, RichText};
use rand::Rng;
pub enum TeamLocation {
    Home,
    Away,
}
#[derive(Debug, PartialEq)]
enum CoinToss {
    Waiting,
    Heads,
    Tails,
}
enum TossResult {
    Waiting,
    Win,
    Lose,
}
#[derive(Debug, PartialEq,Clone, Copy)]
pub enum TeamRoles {
    Waiting,
    Bat,
    Bowl,
}

pub struct Scoreboard {
    game: Game,
    home_team_selector: TeamSelector,
    away_team_selector: TeamSelector,
    teams_selected: bool,

    home_team: String,
    away_team: String,

    home_toss: TossResult,
    away_toss: TossResult,
    completed_coin_toss: bool,

    home_role: TeamRoles,
    away_role: TeamRoles,
    role_chosen: bool,
}
impl Scoreboard {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: Game::new(),
            home_team_selector: TeamSelector::new(),
            away_team_selector: TeamSelector::new(),
            teams_selected: false,

            home_team: "".to_owned(),
            away_team: "".to_owned(),

            home_toss: TossResult::Waiting,
            away_toss: TossResult::Waiting,
            completed_coin_toss: false,

            home_role: TeamRoles::Waiting,
            away_role: TeamRoles::Waiting,
            role_chosen: false,
        }
    }
}
impl eframe::App for Scoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.check_teams_selected() {
            create_team_selector(ctx, self);
        } else if !self.check_completed_coin_toss() {
            coin_toss(ctx, self);
        } else if !self.check_role_chosen() {
            chose_team_role(ctx, self);
        } else {
            let ht = (self.home_team.as_str(), &self.home_role);
            let at = (self.away_team.as_str(), &self.away_role);
            let toss_winner = match self.away_toss {
                TossResult::Win => {
                    (self.away_team.as_str(), &self.away_role)}
                TossResult::Lose => {(self.home_team.as_str(), &self.home_role)}
                TossResult::Waiting => {("",&TeamRoles::Waiting)}
            };
            if !self.game.return_game_finished() {
                self.game.play_match(ctx, ht, at, toss_winner);
            }
        }
    }
}

impl Scoreboard {
    fn set_teams_selected(&mut self) {
        match self.teams_selected {
            true => self.teams_selected = false,
            false => self.teams_selected = true,
        }
    }
    fn set_completed_coin_toss(&mut self, bool: bool) {
        self.completed_coin_toss = bool;
    }
    fn set_role_chosen(&mut self, bool: bool) {
        self.role_chosen = bool;
    }
    fn check_role_chosen(&self) -> bool {
        self.role_chosen
    }
    fn check_completed_coin_toss(&self) -> bool {
        self.completed_coin_toss
    }
    fn check_teams_selected(&self) -> bool {
        self.teams_selected
    }
    fn check_teams_different(&self) -> bool {
        if self.home_team != self.away_team {
            return true;
        } else {
            return false;
        }
    }
    fn check_button_enabled(&self) -> bool {
        let sides_picked = self.home_team_selector.return_team_selected()
            && self.away_team_selector.return_team_selected();
        if sides_picked && self.check_teams_different() {
            return true;
        } else {
            return false;
        }
    }
}

fn create_team_selector(ctx: &egui::Context, sb: &mut Scoreboard) {
    egui::Window::new("Select Teams")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            ui.columns_const(|[t1, t2]| {
                sb.home_team_selector.show_team_select(t1, "id_t1");
                let ht = sb.home_team_selector.return_team();
                sb.home_team = ht.clone();
                t1.label(create_team_labels(TeamLocation::Home, &sb.home_team));

                sb.away_team_selector.show_team_select(t2, "id_t2");
                let at = sb.away_team_selector.return_team();
                sb.away_team = at.clone();
                t2.label(create_team_labels(TeamLocation::Away, &sb.away_team));
            });

            if ui
                .add_enabled(sb.check_button_enabled(), Button::new("Confirm Selections"))
                .clicked()
            {
                sb.set_teams_selected();
            }
        });
}
pub fn create_team_labels(home_away: TeamLocation, team: &str) -> RichText {
    let mut team_label = String::new();
    match home_away {
        TeamLocation::Home => {
            team_label.push_str(&("Home Team: ".to_owned() + team));
        }
        TeamLocation::Away => {
            team_label.push_str(&("Away Team: ".to_owned() + team));
        }
    }

    RichText::new(team_label)
        .color(Color32::WHITE)
        .monospace()
        .size(12.0)
}
fn coin_toss(ctx: &egui::Context, sb: &mut Scoreboard) {
    egui::Window::new("Call Heads or Tails")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            let mut side_picked = CoinToss::Waiting;
            ui.label(
                RichText::new(sb.away_team.clone() + " to call!")
                    .color(Color32::WHITE)
                    .monospace()
                    .size(20.0),
            );
            ui.columns_const(|[heads, tails]| {
                if heads
                    .add(Button::new(format!("{:?}", CoinToss::Heads)))
                    .clicked()
                {
                    side_picked = CoinToss::Heads;
                }
                if tails
                    .add(Button::new(format!("{:?}", CoinToss::Tails)))
                    .clicked()
                {
                    side_picked = CoinToss::Tails;
                }
            });
            if check_coinside_selected(&side_picked) {
                let result: CoinToss = flip_coin();
                if result == side_picked {
                    sb.away_toss = TossResult::Win;
                    sb.home_toss = TossResult::Lose;
                } else {
                    sb.away_toss = TossResult::Lose;
                    sb.home_toss = TossResult::Win;
                }
                sb.set_completed_coin_toss(true);
            }
        });
}
fn flip_coin() -> CoinToss {
    let coin_flip = rand::rng().random_range(1..=100);
    let coin: CoinToss;

    if coin_flip % 2 == 0 {
        coin = CoinToss::Heads
    } else {
        coin = CoinToss::Tails
    }
    coin
}
fn check_coinside_selected(coin: &CoinToss) -> bool {
    match coin {
        CoinToss::Waiting => return false,
        _ => return true,
    }
}
fn chose_team_role(ctx: &egui::Context, sb: &mut Scoreboard) {
    egui::Window::new("Choose to Bat or Bowl")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            let team= match sb.away_toss {
                TossResult::Win => {
                    &sb.away_team
                }
                TossResult::Lose => {
                    &sb.home_team
                }
                TossResult::Waiting => {""}
            };
            let team_label = RichText::new("Toss won by: ".to_owned() + &team)
                            .color(Color32::WHITE)
                            .monospace()
                            .size(20.0);
            ui.label(team_label);
            ui.columns_const(|[bat, bowl]| {
                
                    if bat
                        .add(Button::new(format!("{:?}", TeamRoles::Bat)))
                        .clicked()
                    {
                        match sb.away_toss {
                            TossResult::Win => {
                                sb.away_role = TeamRoles::Bat;
                                sb.home_role = TeamRoles::Bowl;
                            }
                            TossResult::Lose => {
                                sb.away_role = TeamRoles::Bowl;
                                sb.home_role = TeamRoles::Bat;
                            }
                            TossResult::Waiting => {}
                        }
                        sb.set_role_chosen(true);
                    }
                    if bowl
                        .add(Button::new(format!("{:?}", TeamRoles::Bowl)))
                        .clicked()
                    {
                        match sb.away_toss {
                            TossResult::Win => {
                                sb.away_role = TeamRoles::Bowl;
                                sb.home_role = TeamRoles::Bat;
                            }
                            TossResult::Lose => {
                                sb.away_role = TeamRoles::Bat;
                                sb.home_role = TeamRoles::Bowl;
                            }
                            TossResult::Waiting => {}
                        }
                        sb.set_role_chosen(true);
                    }
            });
        });
}
