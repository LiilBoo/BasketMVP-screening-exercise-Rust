use std::collections::HashSet;

#[derive(Debug)]
pub struct Pronostics {
    pub champion: Player,
    pub possible_champions: Vec<Player>,
}

impl Default for Pronostics {
    fn default() -> Self {
        return Self {
            champion: Player::default(),
            possible_champions: Vec::<Player>::new(),
        };
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct Player {
    pub ratio: u16,
    pub age: u8,
    pub name: String,
}

mod helpers {
    use super::Player;
    pub fn is_stronger(target: &Player, other: &Player) -> bool {
        return target.ratio > other.ratio;
    }

    pub fn is_younger(target: &Player, other: &Player) -> bool {
        return target.age < other.age;
    }

    pub fn are_equal_players(target: &Player, other: &Player) -> bool {
        return target.ratio == other.ratio && target.age == other.age;
    }
}

pub fn get_pronostics(player_list: Vec<Player>) -> Pronostics {
    let mut pronostics = Pronostics::default();

    for player in player_list {
        if pronostics
            .possible_champions
            .iter()
            .any(|possible_champion| !helpers::is_stronger(possible_champion, &player))
        {
            pronostics.champion = player.clone();
            pronostics.possible_champions = Vec::new();
            continue;
        }
        if helpers::are_equal_players(&pronostics.champion, &player) {
            pronostics
                .possible_champions
                .push(pronostics.champion.clone());
            pronostics.possible_champions.push(player.clone());
            // pronostics.possible_champions : Vec<ChessPlayer> = BTreeSet::from(pronostics
            //     .possible_champions)
            //     .into_iter().collect()
            //----------
            // pronostics.possible_champions = pronostics.possible_champions.iter().unique();
            //-----------------
            // pronostics.possible_champions = pronostics.possible_champions.sort().dedup();
            //-----------
            pronostics.possible_champions = Vec::from_iter::<HashSet<Player>>(HashSet::from_iter(
                pronostics.possible_champions,
            ));

            continue;
        };

        if !helpers::is_stronger(&pronostics.champion, &player)
            && !helpers::is_younger(&pronostics.champion, &player)
        {
            pronostics.champion = player.clone();
            continue;
        }

        if !helpers::is_stronger(&pronostics.champion, &player) {
            pronostics.champion = player.clone();
            continue;
        }
    }

    return pronostics;
} // end of get_pronostics

#[cfg(test)]
mod tests {
    use crate::{get_pronostics, Player, Pronostics};

    #[test]
    fn returns_player_with_highest_ratio() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3100,
                age: 33,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 32,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.champion.name, "Kareem");
    }
    #[test]
    fn returns_player_with_highest_ratio_and_youngest_age() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3000,
                age: 33,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 32,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.champion.name, "Lebron");
    }
    #[test]
    fn a_list_with_multiple_possible_champions_cant_be_empty() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_ne!(result.possible_champions, Vec::<Player>::new());
        assert!(!result.possible_champions.is_empty());
    }

    #[test]
    fn cannot_return_a_default_object_with_default_values() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];

        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_ne!(result.champion, Player::default());
    }

    #[test]
    fn returns_all_players_with_highest_ratio_and_same_age() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 2900,
                age: 30,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];

        let possible_champions = vec![
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.possible_champions, possible_champions);
    }

    #[test]
    fn returns_all_3_players_with_highest_ratio_and_same_age() {
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 4000,
                age: 32,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 4000,
                age: 32,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 4000,
                age: 32,
                name: String::from("Boo"),
            },
            Player {
                ratio: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            Player {
                ratio: 700,
                age: 30,
                name: String::from("Félix"),
            },
            Player {
                ratio: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            Player {
                ratio: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];

        let possible_champions = vec![
            Player {
                ratio: 4000,
                age: 30,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 4000,
                age: 30,
                name: String::from("Lebron"),
            },
            Player {
                ratio: 4000,
                age: 30,
                name: String::from("Boo"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.possible_champions, possible_champions);
    }
}
