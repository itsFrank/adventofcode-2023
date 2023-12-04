use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

pub fn parse_sample(lines: &Vec<String>) -> Vec<(Vec<i32>, Vec<i32>)> {
    return lines
        .iter()
        .map(|line| {
            let numbers_split: Vec<&str> =
                line.split(":").nth(1).unwrap_or("|").split("|").collect();
            let front_numbers: Vec<i32> = numbers_split
                .get(0)
                .unwrap_or(&"")
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or(-1))
                .filter(|n| *n >= 0)
                .collect();
            let back_numbers: Vec<i32> = numbers_split
                .get(1)
                .unwrap_or(&"")
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or(-1))
                .filter(|n| *n >= 0)
                .collect();
            return (front_numbers, back_numbers);
        })
        .collect();
}

pub fn count_winners(card: Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    card.iter()
        .map(|card| {
            let winner_set: HashSet<i32> = card.0.iter().cloned().collect();
            let num_winners: u32 = card
                .1
                .iter()
                .filter(|n| winner_set.contains(n))
                .count()
                .try_into()
                .unwrap_or(0);
            match num_winners {
                0 => 0,
                n => 2_i32.pow(n - 1),
            }
        })
        .sum()
}

pub fn count_copies(card: Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let wins: Vec<i32> = card
        .iter()
        .map(|card| {
            let winner_set: HashSet<i32> = card.0.iter().cloned().collect();
            card.1
                .iter()
                .filter(|n| winner_set.contains(n))
                .count()
                .try_into()
                .unwrap_or(0)
        })
        .collect();

    let mut copies = vec![1; wins.len()];

    for (i, num_wins) in wins.iter().enumerate() {
        println!("#{} - {} copies and {} wins", i, copies[i], wins[i]);
        for j in i + 1
            ..=i.checked_add((*num_wins as u32).try_into().unwrap())
                .unwrap()
        {
            if j >= copies.len() {
                break;
            }
            println!("\tadd {} copies of {}", copies[i], j);
            copies[j] += copies[i];
        }
    }
    return copies.iter().sum();
}

#[cfg(test)]
mod tests {
    use aoc_shared::{get_resource_lines, resource_path};
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn parse_sample_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let cards = parse_sample(&lines);
        #[rustfmt::skip] assert_eq!(cards[0],(vec![41,48,83,86,17],vec![83,86,6,31,17,9,48,53]));
        #[rustfmt::skip] assert_eq!(cards[1],(vec![13,32,20,16,61],vec![61,30,68,82,17,32,24,19]));
        #[rustfmt::skip] assert_eq!(cards[2],(vec![1,21,53,59,44],vec![69,82,63,72,16,21,14,1]));
        #[rustfmt::skip] assert_eq!(cards[3],(vec![41,92,73,84,69],vec![59,84,76,51,58,5,54,83]));
        #[rustfmt::skip] assert_eq!(cards[4],(vec![87,83,26,28,32],vec![88,30,70,12,93,22,82,36]));
        #[rustfmt::skip] assert_eq!(cards[5],(vec![31,18,13,56,72],vec![74,77,10,23,35,67,36,11]));
    }

    #[test]
    fn sample_part1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let cards = parse_sample(&lines);
        assert_eq!(count_winners(cards), 13);
    }

    #[test]
    fn main_part1_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let cards = parse_sample(&lines);
        assert_eq!(count_winners(cards), 26426);
    }

    #[test]
    fn sample_part2_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let cards = parse_sample(&lines);
        assert_eq!(count_copies(cards), 30);
    }

    #[test]
    fn main_part2_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let cards = parse_sample(&lines);
        assert_eq!(count_copies(cards), 26426);
    }
}
