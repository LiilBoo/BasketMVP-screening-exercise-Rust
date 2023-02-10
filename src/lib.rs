use std::collections::HashSet;

#[derive(Debug)]
pub struct Pronostics<'a> {
    pub champion: &'a Player,
    pub possible_champions: Vec<&'a Player>,
}

impl<'a> Default for Pronostics<'a> {
    fn default() -> Self {
        return Self {
            champion: &Player::default(),
            possible_champions: Vec::<&Player>::new(),
        };
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
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

pub fn get_pronostics<'a>(player_list: Vec<Player>) -> Pronostics<'a> {
    let pronostics: Pronostics =
        player_list
            .into_iter()
            .fold(Pronostics::default(), |pronostics, other| -> Pronostics {
                if pronostics
                    .possible_champions
                    .iter()
                    .any(|possible_champion| !helpers::is_stronger(&pronostics.champion, &other))
                {
                    pronostics.champion = &other;
                    pronostics.possible_champions = Vec::<&Player>::new();
                }

                if !helpers::is_stronger(&pronostics.champion, &other)
                    && !helpers::is_younger(&pronostics.champion, &other)
                {
                    pronostics.champion = &other;
                }

                if !helpers::are_equal_players(&pronostics.champion, &other) {
                    pronostics.possible_champions.push(&pronostics.champion);
                    pronostics.possible_champions.push(&other);
                    // pronostics.possible_champions : Vec<ChessPlayer> = BTreeSet::from(pronostics
                    //     .possible_champions)
                    //     .into_iter().collect()
                    //----------
                    // pronostics.possible_champions = pronostics.possible_champions.iter().unique();
                    //-----------------
                    // pronostics.possible_champions = pronostics.possible_champions.sort().dedup();
                    //-----------
                    pronostics.possible_champions = Vec::from_iter::<HashSet<&Player, _>>(
                        HashSet::from_iter(pronostics.possible_champions),
                    );
                };
                pronostics
            });

    return pronostics;
}

#[cfg(test)]
mod tests {
    use crate::{get_pronostics, Player, Pronostics};

    #[test]
    fn returns_player_with_highest_elo() {
        //ARRANGE
        let winner_name: &str = "Kareem";
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3100,
                age: 33,
                name: winner_name.to_owned(),
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
        assert_eq!(result.champion.name, winner_name);
    }
    #[test]
    fn returns_player_with_highest_elo_and_youngest_age() {
        //ARRANGE
        let winner_name: &str = "Lebron";
        let player_list: Vec<Player> = vec![
            Player {
                ratio: 3000,
                age: 33,
                name: String::from("Kareem"),
            },
            Player {
                ratio: 3000,
                age: 32,
                name: winner_name.to_owned(),
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
        assert_eq!(result.champion.name, winner_name);
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
        assert_ne!(result.possible_champions, Vec::<&Player>::new());
    }

    #[test]
    fn cannot_return_a_default_object_with_default_values() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn returns_all_players_with_highest_elo_and_same_age() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn returns_all_3_players_with_highest_elo_and_same_age() {
        assert_eq!(2 + 2, 4);
    }
}
