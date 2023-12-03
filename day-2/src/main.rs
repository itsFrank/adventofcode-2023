use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type Hand = HashMap<String, i32>;

pub struct Game {
    pub id: usize,
    pub hands: Vec<Hand>,
}

pub fn parse_game(line: &str) -> Game {
    let mut colon_split = line.split(':');
    let game_title = colon_split.next().unwrap();
    let hand_strs = colon_split.next().unwrap().split(";");

    let mut game = Game {
        id: game_title
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(),
        hands: Vec::new(),
    };

    for hand_str in hand_strs {
        let mut hand = Hand::new();
        for cube_str in hand_str.split(",") {
            let trimmed_cube_str = cube_str.trim();
            let mut cube_split = trimmed_cube_str.split_whitespace();
            let count = cube_split.next().unwrap().parse().unwrap();
            let color = cube_split.next().unwrap().to_string();
            hand.insert(color, count);
        }
        game.hands.push(hand);
    }

    return game;
}
pub fn parse_games(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::<Game>::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        games.push(parse_game(&line));
    }
    return games;
}

pub fn sum_possible(max_counts: &Hand, games: &Vec<Game>) -> usize {
    let mut possible_sum: usize = 0;
    for game in games {
        let mut is_possible = true;
        for (key, value) in max_counts {
            for hand in &game.hands {
                if !hand.contains_key(key) {
                    continue;
                }
                if hand[key] > *value {
                    is_possible = false;
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        if is_possible {
            possible_sum += game.id;
        }
    }
    return possible_sum;
}
pub fn sum_min_power(games: &Vec<Game>) -> i32 {
    let mut min_power_sum = 0;
    for game in games {
        let mut min_cubes = Hand::new();
        min_cubes.insert("red".to_string(), 0);
        min_cubes.insert("green".to_string(), 0);
        min_cubes.insert("blue".to_string(), 0);

        for hand in &game.hands {
            for (color, value) in hand {
                if value > &min_cubes[color] {
                    min_cubes.insert(color.to_string(), *value);
                }
            }
        }
        let mut power = 1;
        println!(
            "min: red {}, green {}, blue {}",
            min_cubes["red"],
            min_cubes["green"],
            min_cubes["blue"]
        );
        for (_, value) in min_cubes {
            power *= value;
        }

        min_power_sum += power;
    }
    return min_power_sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::read_file;
    use aoc_shared::resource_path;
    use pretty_assertions::assert_eq;

    #[test]
    fn game_parse_test() {
        let game_str = "Game 15: 3 red, 1 green; 1 blue, 2 yellow";
        let game = parse_game(game_str);
        assert_eq!(game.id, 15);

        let game1_in = &game.hands[0];
        let game1_exp = make_hand(&[("red", 3), ("green", 1)]);
        match_hands(game1_in, &game1_exp);

        let game2_in = &game.hands[1];
        let game2_exp = make_hand(&[("blue", 1), ("yellow", 2)]);
        match_hands(game2_in, &game2_exp);
    }

    #[test]
    fn sample1_test() {
        let lines = get_resource_lines("sample1.txt");
        // do stuff
        let games = parse_games(lines);
        assert_eq!(
            8,
            sum_possible(
                &make_hand(&[("red", 12), ("green", 13), ("blue", 14)]),
                &games
            )
        );
    }

    #[test]
    fn main1_test() {
        let lines = get_resource_lines("main1.txt");
        // do stuff
        let games = parse_games(lines);
        assert_eq!(
            2727,
            sum_possible(
                &make_hand(&[("red", 12), ("green", 13), ("blue", 14)]),
                &games
            )
        );
    }

    #[test]
    fn sample2_test() {
        let lines = get_resource_lines("sample1.txt");
        // do stuff
        let games = parse_games(lines);
        assert_eq!(2286, sum_min_power(&games));
    }

    #[test]
    fn main2_test() {
        let lines = get_resource_lines("main1.txt");
        // do stuff
        let games = parse_games(lines);
        assert_eq!(8, sum_min_power(&games));
    }
    fn make_hand(pairs: &[(&str, i32)]) -> Hand {
        let mut hand = Hand::new();
        for (key, value) in pairs {
            hand.insert(key.to_string(), *value);
        }
        return hand;
    }
    fn assert_hand_value(hand: &Hand, key: &str, exp: i32) {
        assert_eq!(
            format!("hand[{}] = {}", key, exp),
            format!("hand[{}] = {}", key, hand[key])
        );
    }

    fn match_hands(hand_in: &Hand, hand_exp: &Hand) {
        for (key, _) in hand_in {
            assert_eq!(
                "",
                if hand_exp.contains_key(key) {
                    "".to_string()
                } else {
                    format!("missing key: \"{}\"", key)
                }
            );
        }

        for (key, value) in hand_exp {
            assert_hand_value(hand_in, key, *value);
        }
    }

    fn get_resource_lines(res_path: &str) -> Vec<String> {
        let path = resource_path!(res_path).unwrap();
        return read_file(&path)
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect();
    }
}
