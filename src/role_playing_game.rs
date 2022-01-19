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
}

struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,
}

impl Player {
    fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else if self.level > 10 {
            Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            })
        } else {
            Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            })
        }
    }
}
