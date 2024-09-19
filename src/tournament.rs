use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, Default)]
struct Score {
    w: u8,
    d: u8,
    l: u8,
    p: u8,
}

impl Score {
    fn winner() -> Score {
        Score {
            w: 1,
            d: 0,
            l: 0,
            p: 3,
        }
    }

    fn loser() -> Score {
        Score {
            w: 0,
            d: 0,
            l: 1,
            p: 0,
        }
    }
    fn drawer() -> Score {
        Score {
            w: 0,
            d: 1,
            l: 0,
            p: 1,
        }
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Self) -> Self::Output {
        let w = self.w + rhs.w;
        let d = self.d + rhs.d;
        let l = self.l + rhs.l;
        let p = self.p + rhs.p;
        Self { w, d, l, p }
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.w += rhs.w;
        self.d += rhs.d;
        self.l += rhs.l;
        self.p += rhs.p;
    }
}

pub fn tally(match_input: &str) -> String {
    let init: HashMap<&str, Score> = HashMap::new();
    let binding = match_input
        .split("\n")
        .map(|x| {
            let result: Vec<&str> = x.split(";").collect();
            result
        })
        .fold(init, |mut acc, cur| {
            if let [team1, team2, result] = cur[..] {
                if result == "win" {
                    let score = acc.entry(team1).or_default();
                    *score += Score::winner();
                    let score = acc.entry(team2).or_default();
                    *score += Score::loser();
                } else if result == "loss" {
                    let score = acc.entry(team2).or_default();
                    *score += Score::winner();
                    let score = acc.entry(team1).or_default();
                    *score += Score::loser();
                } else {
                    let score = acc.entry(team2).or_default();
                    *score += Score::drawer();
                    let score = acc.entry(team1).or_default();
                    *score += Score::drawer();
                }
            }
            acc
        });
    let mut vecs: Vec<_> = binding.iter().collect();
    vecs.sort_by(|a, b| b.1.p.cmp(&a.1.p).then_with(|| a.0.cmp(b.0)));
    vecs.into_iter().fold(
        String::from("Team                           | MP |  W |  D |  L |  P"),
        |acc, cur| {
            let (team, score) = cur;
            let play = score.w.add(score.d).add(score.l).to_string();
            let win = score.w.to_string();
            let draw = score.d.to_string();
            let lose = score.l.to_string();
            let point = score.w.mul(3).add(score.d).to_string();
            let arr = format!(
                "{:31}|  {play} |  {win} |  {draw} |  {lose} |  {point}",
                team
            );
            acc.add("\n").add(&arr)
        },
    )
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
