#![feature(iter_array_chunks)]

use std::{cmp, collections::HashSet, usize};

fn main() {
    todo!()
}

pub type ParsedMap = (String, Vec<(usize, usize, usize)>);
pub fn parse_problem(lines: &Vec<String>) -> (Vec<usize>, Vec<ParsedMap>) {
    //(seeds, maps)
    let seeds: Vec<usize> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let map_lines: Vec<ParsedMap> = lines[2..lines.len()]
        .iter()
        .fold(Vec::<ParsedMap>::new(), |mut acc, line| {
            if line.is_empty() {
                acc.push(("".to_string(), Vec::<_>::new()))
            } else {
                if line.contains(':') {
                    let title = line.split_whitespace().next().unwrap();
                    if acc.is_empty() {
                        acc.push(("".to_string(), Vec::<_>::new()))
                    }
                    acc.last_mut().unwrap().0 = title.to_string();
                } else {
                    let values: Vec<usize> = line
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect();

                    if values.len() != 3 {
                        panic!("should have 3 numbers in string: \"{}\"", line);
                    }
                    acc.last_mut()
                        .unwrap()
                        .1
                        .push((values[0], values[1], values[2]));
                }
            }
            acc
        })
        .into_iter()
        .filter(|m| !m.0.is_empty())
        .collect();

    return (seeds, map_lines);
}

pub fn find_mapping(map: &Vec<(usize, usize, usize)>, value: usize) -> usize {
    for (dst, src, len) in map {
        if value >= *src && value <= src + len {
            return dst + value.checked_sub(*src).unwrap_or(0);
        }
    }
    return value;
}

pub fn chain_mappings(seeds: &Vec<usize>, mappings: Vec<ParsedMap>) -> Vec<usize> {
    return seeds
        .iter()
        .map(|s| {
            mappings
                .iter()
                .fold(*s, |acc, mapping| find_mapping(&mapping.1, acc))
        })
        .collect();
}

pub fn seed_vec_to_ranges(seeds: &Vec<usize>) -> Vec<(usize, usize)> {
    seeds
        .iter()
        .array_chunks::<2>()
        .map(|pair| (*pair[0], (pair[0] + pair[1]).checked_sub(1).unwrap()))
        .collect()
}

pub fn range_matcher(
    range_in: (usize, usize),
    transform: (usize, usize, usize),
) -> Vec<(usize, usize)> {
    fn map_range(
        range_in: (usize, usize),
        range_src: (usize, usize),
        range_dst: (usize, usize),
    ) -> (usize, usize) {
        (
            range_dst.0 + range_in.0.checked_sub(range_src.0).unwrap_or(0),
            range_dst.0 + range_in.1.checked_sub(range_src.0).unwrap_or(0),
        )
    }

    let delta_incl = transform.2.checked_sub(1).unwrap();
    let range_src = (transform.1, transform.1 + delta_incl);
    let range_dst = (transform.0, transform.0 + delta_incl);

    let range_left = if range_in.0 < range_src.0 {
        Some((
            range_in.0,
            cmp::min(range_in.1, range_src.0.checked_sub(1).unwrap()),
        ))
    } else {
        None
    };

    let range_right = if range_in.1 > range_src.1 {
        Some((cmp::max(range_in.0, range_src.1 + 1), range_in.1))
    } else {
        None
    };

    let range_middle = if (range_in.0 >= range_src.0 && range_in.0 <= range_src.1)
        || (range_in.1 >= range_src.0 && range_in.1 <= range_src.1)
        || (range_in.0 <= range_src.0 && range_in.1 >= range_src.1)
    {
        Some(map_range(
            (
                cmp::max(range_in.0, range_src.0),
                cmp::min(range_in.1, range_src.1),
            ),
            range_src,
            range_dst,
        ))
    } else {
        None
    };

    let ranges = vec![range_left, range_right, range_middle];
    return ranges
        .iter()
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();
}

pub fn range_overlap(
    range_in: (usize, usize),
    range_src: (usize, usize),
) -> Option<(usize, usize)> {
    if (range_in.0 >= range_src.0 && range_in.0 <= range_src.1)
        || (range_in.1 >= range_src.0 && range_in.1 <= range_src.1)
        || (range_in.0 <= range_src.0 && range_in.1 >= range_src.1)
    {
        Some((
            cmp::max(range_in.0, range_src.0),
            cmp::min(range_in.1, range_src.1),
        ))
    } else {
        None
    }
}

pub fn ranges_outside(range_in: (usize, usize), range_src: (usize, usize)) -> Vec<(usize, usize)> {
    let range_left = if range_in.0 < range_src.0 {
        Some((
            range_in.0,
            cmp::min(range_in.1, range_src.0.checked_sub(1).unwrap()),
        ))
    } else {
        None
    };

    let range_right = if range_in.1 > range_src.1 {
        Some((cmp::max(range_in.0, range_src.1 + 1), range_in.1))
    } else {
        None
    };

    let ranges = vec![range_left, range_right];
    return ranges
        .iter()
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();
}

pub fn range_map(
    range_in: (usize, usize),
    transform: (usize, usize, usize),
) -> Option<(usize, usize)> {
    fn map_range(
        range_in: (usize, usize),
        range_src: (usize, usize),
        range_dst: (usize, usize),
    ) -> (usize, usize) {
        (
            range_dst.0 + range_in.0.checked_sub(range_src.0).unwrap_or(0),
            range_dst.0 + range_in.1.checked_sub(range_src.0).unwrap_or(0),
        )
    }

    let delta_incl = transform.2.checked_sub(1).unwrap();
    let range_src = (transform.1, transform.1 + delta_incl);
    let range_dst = (transform.0, transform.0 + delta_incl);

    let range_middle = if (range_in.0 >= range_src.0 && range_in.0 <= range_src.1)
        || (range_in.1 >= range_src.0 && range_in.1 <= range_src.1)
        || (range_in.0 <= range_src.0 && range_in.1 >= range_src.1)
    {
        Some(map_range(
            (
                cmp::max(range_in.0, range_src.0),
                cmp::min(range_in.1, range_src.1),
            ),
            range_src,
            range_dst,
        ))
    } else {
        None
    };

    return range_middle;
}

pub fn map_ranges(ranges_in: &Vec<(usize, usize)>, mapping: &ParsedMap) -> Vec<(usize, usize)> {
    fn src_range(transform: (usize, usize, usize)) -> (usize, usize) {
        return (
            transform.1,
            transform.1 + transform.2.checked_sub(1).unwrap(),
        );
    }

    let matched_ranges: Vec<(usize, usize)> = mapping
        .1
        .iter()
        .flat_map(|m| {
            ranges_in
                .iter()
                .map(|r| range_overlap(*r, src_range(*m)))
                .filter(|v| v.is_some())
                .map(|v| v.unwrap())
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let missed_ranges: Vec<(usize, usize)> = ranges_in
        .iter()
        .flat_map(|r| {
            matched_ranges.iter().fold(vec![*r], |acc, m| {
                acc.iter().flat_map(|a| ranges_outside(*a, *m)).collect()
            })
        })
        .collect();

    let mapped_ranges: Vec<(usize, usize)> = mapping
        .1
        .iter()
        .flat_map(|m| {
            matched_ranges
                .iter()
                .map(|r| range_map(*r, *m))
                .filter(|v| v.is_some())
                .map(|v| v.unwrap())
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    return [mapped_ranges, missed_ranges].concat();
}

pub fn chain_map_ranges(
    seed_ranges: &Vec<(usize, usize)>,
    mappings: Vec<ParsedMap>,
) -> Vec<(usize, usize)> {
    mappings
        .iter()
        .fold(seed_ranges.clone(), |acc, m| map_ranges(&acc, m))
}

pub fn chain_range_mappings(
    seed_ranges: &Vec<(usize, usize)>,
    mappings: Vec<ParsedMap>,
) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = seed_ranges
        .iter()
        .flat_map(|range| {
            mappings.iter().fold(vec![*range], |acc, mapping| {
                acc.iter()
                    .flat_map(move |&r| mapping.1.iter().flat_map(move |&m| range_matcher(r, m)))
                    .collect()
            })
        })
        .collect();

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    ranges.retain(|r| set.insert(r.clone()));

    return ranges;
}

pub fn collapse_ranges(ranges: &Vec<(usize, usize)>) {}

#[cfg(test)]
mod tests {
    use aoc_shared::{get_resource_lines, resource_path};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn find_mapping_test() {
        let mapping: Vec<(usize, usize, usize)> = vec![(12, 10, 4), (199, 99, 10)];
        assert_eq!(find_mapping(&mapping, 1), 1);
        assert_eq!(find_mapping(&mapping, 10), 12);
        assert_eq!(find_mapping(&mapping, 11), 13);
        assert_eq!(find_mapping(&mapping, 12), 14);
        assert_eq!(find_mapping(&mapping, 13), 15);
        assert_eq!(find_mapping(&mapping, 14), 16);
        assert_eq!(find_mapping(&mapping, 15), 15);

        assert_eq!(find_mapping(&mapping, 98), 98);
        assert_eq!(find_mapping(&mapping, 99), 199);
        assert_eq!(find_mapping(&mapping, 109), 209);
        assert_eq!(find_mapping(&mapping, 110), 110);

        let mapping: Vec<(usize, usize, usize)> = vec![(50, 98, 2), (52, 50, 48)];
        assert_eq!(find_mapping(&mapping, 0), 0);
        assert_eq!(find_mapping(&mapping, 1), 1);
        assert_eq!(find_mapping(&mapping, 48), 48);
        assert_eq!(find_mapping(&mapping, 49), 49);
        assert_eq!(find_mapping(&mapping, 50), 52);
        assert_eq!(find_mapping(&mapping, 51), 53);
        assert_eq!(find_mapping(&mapping, 96), 98);
        assert_eq!(find_mapping(&mapping, 97), 99);
        assert_eq!(find_mapping(&mapping, 98), 50);
        assert_eq!(find_mapping(&mapping, 99), 51);
    }

    #[test]
    fn sample_parse_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem(&lines);
        assert_eq!(parsed.0, vec![79, 14, 55, 13]);

        assert_eq!(parsed.1[0].0, "seed-to-soil");
        assert_eq!(parsed.1[0].1, vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(parsed.1[1].0, "soil-to-fertilizer");
        assert_eq!(parsed.1[1].1, vec![(0, 15, 37), (37, 52, 2), (39, 0, 15),]);

        assert_eq!(parsed.1[2].0, "fertilizer-to-water");
        assert_eq!(
            parsed.1[2].1,
            vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4),]
        );

        assert_eq!(parsed.1[3].0, "water-to-light");
        assert_eq!(parsed.1[3].1, vec![(88, 18, 7), (18, 25, 70),]);

        assert_eq!(parsed.1[4].0, "light-to-temperature");
        assert_eq!(
            parsed.1[4].1,
            vec![(45, 77, 23), (81, 45, 19), (68, 64, 13),]
        );

        assert_eq!(parsed.1[5].0, "temperature-to-humidity");
        assert_eq!(parsed.1[5].1, vec![(0, 69, 1), (1, 0, 69),]);

        assert_eq!(parsed.1[6].0, "humidity-to-location");
        assert_eq!(parsed.1[6].1, vec![(60, 56, 37), (56, 93, 4),]);
    }

    #[test]
    fn sample_part1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem(&lines);
        assert_eq!(
            *chain_mappings(&parsed.0, parsed.1).iter().min().unwrap(),
            35usize
        );
    }

    #[test]
    fn main_part1_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let parsed = parse_problem(&lines);
        assert_eq!(
            *chain_mappings(&parsed.0, parsed.1).iter().min().unwrap(),
            196167384usize
        );
    }

    #[test]
    fn seed_to_range_test() {
        let seeds: Vec<usize> = vec![79, 14, 55, 13];
        let ranges = seed_vec_to_ranges(&seeds);
        assert_eq!(ranges[0], (79, 92));
        assert_eq!(ranges[1], (55, 67));
    }

    #[test]
    fn range_matcher_test() {
        let transform = (101, 11, 11);

        // under
        let range_in = (1, 10);
        let ranges_exp = vec![(1, 10)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // over
        let range_in = (22, 32);
        let ranges_exp = vec![(22, 32)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // equal
        let range_in = (11, 21);
        let ranges_exp = vec![(101, 111)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // within
        let range_in = (12, 20);
        let ranges_exp = vec![(102, 110)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // stradle left
        let range_in = (8, 18);
        let ranges_exp = vec![(8, 10), (101, 108)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // stradle right
        let range_in = (18, 28);
        let ranges_exp = vec![(22, 28), (108, 111)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // surround
        let range_in = (8, 28);
        let ranges_exp = vec![(8, 10), (22, 28), (101, 111)];
        let ranges_got = range_matcher(range_in, transform);
        assert_eq!(ranges_got, ranges_exp);

        // from sample
        let range_in = (55, 67);
        let ranges_exp = vec![(61, 67), (51, 56)];
        let ranges_got = range_matcher(range_in, (49, 53, 8));
        assert_eq!(ranges_got, ranges_exp);
    }

    #[test]
    fn sample2_step_test() {
        let seeds_in: Vec<(usize, usize)> = vec![(79, 92), (55, 67)];

        let seed_to_soil: ParsedMap = ("seed_to_soil".to_string(), vec![(50, 98, 2), (52, 50, 48)]);

        let soil_to_fertilizer: ParsedMap = (
            "soil_to_fertilizer".to_string(),
            vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
        );
        let mut ranges_got = map_ranges(&seeds_in, &soil_to_fertilizer);
        let mut ranges_exp: Vec<(usize, usize)> = vec![(55, 67), (79, 92)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);

        let fertilizer_to_water: ParsedMap = (
            "fertilizer_to_water".to_string(),
            vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
        );
        let mut ranges_got = map_ranges(&ranges_got, &fertilizer_to_water);
        let mut ranges_exp: Vec<(usize, usize)> = vec![(51, 56), (61, 67), (79, 92)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);

        let water_to_light: ParsedMap = (
            "water_to_light".to_string(),
            vec![(88, 18, 7), (18, 25, 70)], // (25, 94) -7
        );
        let mut ranges_got = map_ranges(&ranges_got, &water_to_light);
        let mut ranges_exp: Vec<(usize, usize)> = vec![(44, 49), (54, 60), (72, 85)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);

        let light_to_temperature: ParsedMap = (
            "light_to_temperature".to_string(),
            vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)], // (77, 99), (45, 63), (64, 76)
        );
        let mut ranges_got = map_ranges(&ranges_got, &light_to_temperature);
        let mut ranges_exp: Vec<(usize, usize)> =
            vec![(44, 44), (45, 53), (76, 80), (81, 85), (90, 96)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);

        let temperature_to_humidity: ParsedMap = (
            "temperature_to_humidity".to_string(),
            vec![(0, 69, 1), (1, 0, 69)], // (69, 70), (0, 68)
        );
        let mut ranges_got = map_ranges(&ranges_got, &temperature_to_humidity);
        let mut ranges_exp: Vec<(usize, usize)> =
            vec![(45, 45), (46, 54), (76, 80), (81, 85), (90, 96)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);

        let humidity_to_location: ParsedMap = (
            "humidity_to_location".to_string(),
            vec![(60, 56, 37), (56, 93, 4)], // (56, 92), (93, 96)
        );
        let mut ranges_got = map_ranges(&ranges_got, &humidity_to_location);
        let mut ranges_exp: Vec<(usize, usize)> =
            vec![(45, 45), (46, 54), (80, 84), (85, 89), (94, 96), (56, 59)];
        ranges_exp.sort();
        ranges_got.sort();
        assert_eq!(ranges_got, ranges_exp);
    }

    #[test]
    fn sample_part2_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem(&lines);
        let seed_ranges = seed_vec_to_ranges(&parsed.0);
        let final_ranges = chain_map_ranges(&seed_ranges, parsed.1);
        let result = final_ranges
            .iter()
            .flat_map(|r| vec![r.0, r.1])
            .min()
            .unwrap();
        assert_eq!(result, 46usize);
    }

    #[test]
    fn main_part2_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let parsed = parse_problem(&lines);
        let seed_ranges = seed_vec_to_ranges(&parsed.0);
        let final_ranges = chain_map_ranges(&seed_ranges, parsed.1);
        let result = final_ranges
            .iter()
            .flat_map(|r| vec![r.0, r.1])
            .min()
            .unwrap();
        assert_eq!(result, 125742456usize);
    }
}
