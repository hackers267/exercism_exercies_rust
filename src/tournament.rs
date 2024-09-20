use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul};

#[derive(Default, PartialEq, Eq)]
struct Team {
    name: String,
    matches: u8,
    wins: u8,
    draws: u8,
    losses: u8,
    points: u16,
}

impl Team {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        Self {
            name,
            ..Default::default()
        }
    }
    fn win(&mut self) {
        self.wins += 1;
        self.matches += 1;
        self.points += 3;
    }
    fn draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
    fn loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
    fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Draw => self.draw(),
            MatchResult::Loss => self.loss(),
        }
    }
}

impl From<&Team> for String {
    fn from(team: &Team) -> Self {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team.name, team.matches, team.wins, team.draws, team.losses, team.points
        )
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Draw => MatchResult::Draw,
            MatchResult::Loss => MatchResult::Win,
        }
    }
}

impl From<&str> for MatchResult {
    fn from(value: &str) -> Self {
        match value {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => unreachable!(),
        }
    }
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_input: &str) -> String {
    let init: HashMap<_, _> = HashMap::new();
    let scores = match_input
        .lines()
        .map(|x| x.split(";").collect::<Vec<_>>())
        .fold(init, |mut acc, cur| {
            if let [team1, team2, result] = cur[..] {
                let result = result.into();
                acc.entry(team1.to_string())
                    .or_insert(Team::new(team1))
                    .add_match(&result);
                acc.entry(team2.to_string())
                    .or_insert(Team::new(team2))
                    .add_match(&result.reverse());
            }
            acc
        });
    let mut vecs: Vec<&Team> = scores.values().collect();
    vecs.sort_by(|a, b| b.points.cmp(&a.points).then_with(|| a.name.cmp(&b.name)));
    vec![HEADER.to_string()]
        .into_iter()
        .chain(vecs.into_iter().map(|v| v.into()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_the_header_if_no_input() {
        let input = "";
        let expected = "Team                           | MP |  W |  D |  L |  P";
        assert_eq!(tally(input), expected);
    }
    #[test]
    fn a_win_is_three_points_a_loss_is_zero_points() {
        let input = "Allegoric Alaskans;Blithering Badgers;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
            + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
        assert_eq!(tally(input), expected);
    }
    #[test]
    fn a_win_can_also_be_expressed_as_a_loss() {
        let input = "Blithering Badgers;Allegoric Alaskans;loss";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
            + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
        assert_eq!(tally(input), expected);
    }
    #[test]
    fn a_different_team_can_win() {
        let input = "Blithering Badgers;Allegoric Alaskans;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Blithering Badgers             |  1 |  1 |  0 |  0 |  3\n"
            + "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0";
        assert_eq!(tally(input), expected);
    }
    #[test]
    fn there_can_be_more_than_one_match() {
        let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
            + "Allegoric Alaskans;Blithering Badgers;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
            + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn a_draw_is_one_point_each() {
        let input = "Allegoric Alaskans;Blithering Badgers;draw\n".to_string()
            + "Allegoric Alaskans;Blithering Badgers;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  2 |  1 |  1 |  0 |  4\n"
            + "Blithering Badgers             |  2 |  0 |  1 |  1 |  1";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn there_can_be_more_than_one_winner() {
        let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
            + "Allegoric Alaskans;Blithering Badgers;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3\n"
            + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn there_can_be_more_than_two_teams() {
        let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
            + "Blithering Badgers;Courageous Californians;win\n"
            + "Courageous Californians;Allegoric Alaskans;loss";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
            + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n"
            + "Courageous Californians        |  2 |  0 |  0 |  2 |  0";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn typical_input() {
        let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
            + "Devastating Donkeys;Courageous Californians;draw\n"
            + "Devastating Donkeys;Allegoric Alaskans;win\n"
            + "Courageous Californians;Blithering Badgers;loss\n"
            + "Blithering Badgers;Devastating Donkeys;loss\n"
            + "Allegoric Alaskans;Courageous Californians;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n"
            + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
            + "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n"
            + "Courageous Californians        |  3 |  0 |  1 |  2 |  1";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn incomplete_competition_not_all_pairs_have_played() {
        let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
            + "Devastating Donkeys;Allegoric Alaskans;loss\n"
            + "Courageous Californians;Blithering Badgers;draw\n"
            + "Allegoric Alaskans;Courageous Californians;win";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
            + "Blithering Badgers             |  2 |  1 |  1 |  0 |  4\n"
            + "Courageous Californians        |  2 |  0 |  1 |  1 |  1\n"
            + "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0";
        assert_eq!(tally(&input), expected);
    }
    #[test]
    fn ties_broken_alphabetically() {
        let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
            + "Allegoric Alaskans;Blithering Badgers;win\n"
            + "Devastating Donkeys;Allegoric Alaskans;loss\n"
            + "Courageous Californians;Blithering Badgers;win\n"
            + "Blithering Badgers;Devastating Donkeys;draw\n"
            + "Allegoric Alaskans;Courageous Californians;draw";
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
            + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
            + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
            + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";
        assert_eq!(tally(&input), expected);
    }
}
