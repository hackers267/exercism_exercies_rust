use std::cmp::Ordering;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dead_player() {
        let dead_player = Player {
            health: 0,
            mana: None,
            level: 2,
        };
        let new_player = dead_player.revive().unwrap();
        assert_eq!(new_player.health, 100);
        assert_eq!(new_player.mana, None);
        assert_eq!(new_player.level, 2);
    }

    #[test]
    fn test_dead_player_level_gt_10() {
        let dead_player = Player {
            health: 0,
            mana: None,
            level: 12,
        };
        let new_player = dead_player.revive();
        assert!(new_player.is_some());
        let new_player = new_player.unwrap();
        assert_eq!(new_player.health, 100);
        assert_eq!(new_player.level, 12);
        assert_eq!(new_player.mana, Some(100));
    }

    #[test]
    fn test_alive_player() {
        let alive_player = Player {
            health: 1,
            mana: Some(15),
            level: 11,
        };
        let new_player = alive_player.revive();
        assert!(new_player.is_none());
    }

    #[test]
    fn test_not_wizard_yet() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: None,
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
        assert_eq!(not_a_wizard_yet.health, 74);
        assert_eq!(not_a_wizard_yet.mana, None);
    }

    #[test]
    fn test_low_mana_wizard() {
        let mut low_mana_wizard = Player {
            health: 93,
            mana: Some(3),
            level: 12,
        };
        assert_eq!(low_mana_wizard.cast_spell(10), 0);
        assert_eq!(low_mana_wizard.health, 93);
        assert_eq!(low_mana_wizard.mana, Some(3));
    }

    #[test]
    fn test_wizard() {
        let mut wizard = Player {
            health: 123,
            mana: Some(30),
            level: 18,
        };
        assert_eq!(wizard.cast_spell(10), 20);
        assert_eq!(wizard.health, 123);
        assert_eq!(wizard.mana, Some(20));
    }
}

struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Player {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level,
        }
    }

    fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                match self.health.cmp(&mana_cost) {
                    Ordering::Less => self.health = 0,
                    _ => {
                        self.health = self.health - mana_cost;
                    }
                }
                0
            }
            Some(value) => match value.cmp(&mana_cost) {
                Ordering::Less => 0,
                _ => {
                    self.mana = Some(value - mana_cost);
                    &mana_cost * 2
                }
            },
        }
    }
}
