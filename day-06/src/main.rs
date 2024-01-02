fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
pub struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn new(time: i64, distance: i64) -> Race {
        Race { time, distance }
    }
}

pub fn parse_problem1(lines: &Vec<String>) -> Vec<Race> {
    let times: Vec<i64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    return times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect();
}
pub fn parse_problem2(lines: &Vec<String>) -> Race {
    let mut iter = lines.into_iter().map(|l| {
        l.replace(" ", "")
            .split(":")
            .skip(1)
            .map(|s| s.parse::<i64>().unwrap())
            .next()
            .unwrap()
    });

    return Race::new(iter.next().unwrap(), iter.next().unwrap());
}

pub fn num_ways_to_win(race: Race) -> i64 {
    fn distance(press: i64, total: i64) -> i64 {
        if press <= 0 || press >= total {
            return 0;
        }
        return (press) * (total - press);
    }

    let mut count = 0;
    for press in 1..race.time {
        if distance(press, race.time) > race.distance {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use aoc_shared::{get_resource_lines, resource_path};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem1(&lines);
        let exp = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        assert_eq!(parsed, exp);
    }

    #[test]
    fn sample_part1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem1(&lines);
        let solution = parsed
            .into_iter()
            .map(|r| num_ways_to_win(r))
            .fold(1, |acc, count| acc * count);
        assert_eq!(solution, 288);
    }

    #[test]
    fn main_part1_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let parsed = parse_problem1(&lines);
        let solution = parsed
            .into_iter()
            .map(|r| num_ways_to_win(r))
            .fold(1, |acc, count| acc * count);
        assert_eq!(solution, 131376);
    }

    #[test]
    fn parse2_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem2(&lines);
        let exp = Race::new(71530, 940200);
        assert_eq!(parsed, exp);
    }

    #[test]
    fn sample_part2_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem2(&lines);
        let solution =  num_ways_to_win(parsed);
        assert_eq!(solution, 71503);
    }

    #[test]
    fn main_part2_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let parsed = parse_problem2(&lines);
        let solution =  num_ways_to_win(parsed);
        assert_eq!(solution, 34123437);
    }
}
