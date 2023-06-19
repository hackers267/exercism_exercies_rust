use std::collections::{hash_map, HashMap};
use std::ops::{Add, Mul};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn just_the_header_if_no_input() {
        let input = "";
        let expected = "Team | MP | W | D | L | P";
        assert_eq!(tally(input), expected)
    }

    #[test]
    fn a_win_is_three_points_a_lose_is_zero_points() {
        let input = "A;B;win";
        let expected = "".to_string()
            + "Team | MP | W | D | L | P\n"
            + "A | 1 | 1 | 0 | 0 | 3\n"
            + "B | 1 | 0 | 0 | 1 | 0";
        assert_eq!(tally(input), expected);
    }

    #[test]
    fn a_win_can_also_be_expressed_as_a_loss() {
        let input = "B;A;loss";
        let expected = "".to_string()
            + "Team | MP | W | D | L | P\n"
            + "A | 1 | 1 | 0 | 0 | 3\n"
            + "B | 1 | 0 | 0 | 1 | 0";
        assert_eq!(tally(input), expected);
    }
}

#[derive(Debug)]
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

pub fn tally(match_input: &str) -> String {
    let mut init: HashMap<&str, Score> = HashMap::new();
    match_input
        .split("\n")
        .map(|x| {
            let result: Vec<&str> = x.split(";").collect();
            result
        })
        .fold(init, |mut acc, cur| {
            if let [team1, team2, result] = cur[..] {
                if result == "win" {
                    acc.insert(team1, Score::winner());
                    acc.insert(team2, Score::loser());
                } else if result == "loss" {
                    acc.insert(team1, Score::loser());
                    acc.insert(team2, Score::winner());
                } else {
                    acc.insert(team1, Score::drawer());
                    acc.insert(team2, Score::drawer());
                }
            }
            println!("{:#?}", acc);
            acc
        })
        .iter()
        .fold(String::from("Team | MP | W | D | L | P"), |acc, cur| {
            println!("{:?}", cur);
            let (team, score) = cur;
            let play = score.w.add(score.d).add(score.l).to_string();
            let win = score.w.to_string();
            let draw = score.d.to_string();
            let lose = score.l.to_string();
            let point = score.w.mul(3).add(score.d).to_string();
            let arr = [team.to_string(), play, win, draw, lose, point].join(" | ");
            acc.add("\n").add(&arr)
        })
}
