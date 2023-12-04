use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Part {
    pub id: usize,
    pub value: i32,
}

pub type PartMap = HashMap<(usize, usize), Part>;
trait PartHolder {
    fn insert_part(&mut self, x_range: (usize, usize), y: usize, part: Part);
}

impl PartHolder for PartMap {
    fn insert_part(&mut self, (x_start, x_end): (usize, usize), y: usize, part: Part) {
        for x in x_start..x_end {
            self.insert((x, y), part);
        }
    }
}

pub fn make_part_map(lines: &Vec<String>) -> PartMap {
    let mut part_map = PartMap::new();
    let mut next_id: usize = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut num_string: Option<String> = None;
        let mut num_start: usize = 0;
        for (x, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                match num_string {
                    Some(ref mut s) => {
                        s.push(c);
                    }
                    None => {
                        num_string = Some(c.to_string());
                        num_start = x;
                    }
                }
            } else {
                if let Some(ref s) = num_string {
                    let part = Part {
                        id: next_id,
                        value: s.parse().unwrap(),
                    };
                    part_map.insert_part((num_start, x), y, part);

                    next_id += 1;
                    num_start = 0;
                    num_string = None;
                };
            }
        }

        if let Some(s) = num_string {
            let part = Part {
                id: next_id,
                value: s.parse().unwrap(),
            };
            next_id += 1;
            part_map.insert_part((num_start, line.len()), y, part);
        }
    }

    return part_map;
}

pub fn count_parts(lines: &Vec<String>, part_map: &PartMap) -> i32 {
    let mut part_sum: i32 = 0;
    let mut visited_part_ids = HashSet::<usize>::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' || (c >= '0' && c <= '9') {
                continue;
            }

            for y in y.checked_sub(1).unwrap_or(0)..=y + 1 {
                for x in x.checked_sub(1).unwrap_or(0)..=x + 1 {
                    match part_map.get(&(x, y)) {
                        Some(part) => {
                            if !visited_part_ids.contains(&part.id) {
                                visited_part_ids.insert(part.id);
                                part_sum += part.value;
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }
    return part_sum;
}

pub fn gear_ratios(lines: &Vec<String>, part_map: &PartMap) -> i32 {
    let mut ratio_sum: i32 = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }

            let mut visited_part_ids = HashSet::<usize>::new();
            let mut gears = Vec::<i32>::new();
            for y in y.checked_sub(1).unwrap_or(0)..=y + 1 {
                for x in x.checked_sub(1).unwrap_or(0)..=x + 1 {
                    match part_map.get(&(x, y)) {
                        Some(part) => {
                            if !visited_part_ids.contains(&part.id) {
                                visited_part_ids.insert(part.id);
                                gears.push(part.value);
                            }
                        }
                        None => {}
                    }
                }
            }

            if gears.len() != 2 {
                continue;
            }
            ratio_sum += gears[0] * gears[1];
        }
    }
    return ratio_sum;
}

#[cfg(test)]
mod tests {
    use aoc_shared::{get_resource_lines, resource_path};
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::{count_parts, make_part_map, Part, PartMap, gear_ratios};

    // use super::*;

    #[test]
    fn sample1_part_map_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let part_map = make_part_map(&lines);

        let part_exp = Part { id: 0, value: 467 };
        check_part_for_coords(&part_map, part_exp, (0, 0), (2, 0));

        let part_exp = Part { id: 1, value: 114 };
        check_part_for_coords(&part_map, part_exp, (5, 0), (7, 0));
    }

    #[test]
    fn edge_cases_test() {
        // *@=%+$&/-#
        let lines: Vec<String> = concat!(
            "......1.1.....\n",
            ".......*.......\n",
            "......1.1......\n",
            ".......1.......\n",
            "......1*1......\n",
            ".......1.......\n"
        )
        .split("\n")
        .map(|s| s.to_string())
        .collect();

        let part_map = make_part_map(&lines);
        assert_eq!(count_parts(&lines, &part_map), 8);
    }

    #[test]
    fn sample1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let part_map = make_part_map(&lines);
        assert_eq!(count_parts(&lines, &part_map), 4361);
    }

    #[test]
    fn sample2_test() {
        let lines = get_resource_lines(&resource_path!("sample2.txt").unwrap());
        let part_map = make_part_map(&lines);
        assert_eq!(count_parts(&lines, &part_map), 925);
    }

    #[test]
    fn main1_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let part_map = make_part_map(&lines);
        assert_eq!(count_parts(&lines, &part_map), 507214);
    }

    #[test]
    fn sample_part2_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let part_map = make_part_map(&lines);
        assert_eq!(gear_ratios(&lines, &part_map), 467835);
    }

    #[test]
    fn main_part2_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let part_map = make_part_map(&lines);
        assert_eq!(gear_ratios(&lines, &part_map), 72553319);
    }

    fn check_part_for_coords(
        part_map: &PartMap,
        part_exp: Part,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        for_each_point(start, end, |p| {
            let part: Option<&Part> = part_map.get(&p);
            let msg_prefix = format!("PartMap[({},{})]: ", p.0, p.1);
            let msg_in = format!(
                "{} {}",
                msg_prefix,
                match part {
                    Some(part) => format!("{:#?}", part),
                    None => "None".to_string(),
                }
            );
            let msg_exp = format!("{} {:#?}", msg_prefix, part_exp);
            assert_eq!(msg_in, msg_exp);
        });
    }

    fn for_each_point<F>(start: (usize, usize), end: (usize, usize), mut action: F)
    where
        F: FnMut((usize, usize)),
    {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;

        // Create an iterator over the cartesian product of the ranges
        let points = (start_x..=end_x).flat_map(move |x| (start_y..=end_y).map(move |y| (x, y)));

        // Execute the action for each point
        points.for_each(|point| action(point));
    }
}
