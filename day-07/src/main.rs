use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

#[derive(Debug)]
pub struct Hand {
    cards: [i32; 5],
    bet: i32,
    kind: HandKind,
}

impl Hand {
    pub fn new(cards: [i32; 5], bet: i32, kind: HandKind) -> Hand {
        Hand { cards, bet, kind }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == core::cmp::Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind > other.kind {
            return Some(core::cmp::Ordering::Greater);
        }
        if other.kind > self.kind {
            return Some(core::cmp::Ordering::Less);
        }

        for (mine, theirs) in self.cards.iter().zip(other.cards.iter()) {
            if mine > theirs {
                return Some(core::cmp::Ordering::Greater);
            }
            if theirs > mine {
                return Some(core::cmp::Ordering::Less);
            }
        }
        return Some(core::cmp::Ordering::Equal);
    }
}

pub fn parse_hand(line: &str, use_jokers: bool) -> Hand {
    let char_map: HashMap<char, i32> = [
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        (
            'J',
            match use_jokers {
                false => 11,
                true => -1,
            },
        ),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .into_iter()
    .collect();

    let mut split = line.split_whitespace();
    let hand: [i32; 5] = split
        .next()
        .unwrap()
        .chars()
        .map(|c| char_map.get(&c).unwrap().clone())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let bet = split.next().unwrap_or("").parse::<i32>().unwrap_or(0);
    Hand::new(hand, bet, get_hand_kind(&hand))
}

pub fn parse_problem(lines: &Vec<String>, use_jokers: bool) -> Vec<Hand> {
    lines
        .into_iter()
        .map(|l| parse_hand(l, use_jokers))
        .collect()
}

pub fn get_hand_kind(cards: &[i32; 5]) -> HandKind {
    // Five of a kind, where all five cards have the same label: AAAAA
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // High card, where all cards' labels are distinct: 23456

    let mut card_counts = HashMap::<i32, i32>::new();

    for card in cards {
        let count: i32 = card_counts.get(&card).cloned().unwrap_or(0);
        card_counts.insert(*card, count + 1);
    }

    let joker_count = card_counts.get(&-1).cloned().unwrap_or(0);
    if joker_count == 5 {
        return HandKind::FiveKind;
    }

    let mut pair_count = 0;
    let mut trip_count = 0;
    for (card, count) in card_counts {
        if card == -1 {
            continue;
        }

        match count {
            5 => return HandKind::FiveKind,
            4 => match joker_count {
                0 => return HandKind::FourKind,
                1 => return HandKind::FiveKind,
                _ => unreachable!(),
            },
            3 => match joker_count {
                0 => trip_count += 1,
                1 => return HandKind::FourKind,
                2 => return HandKind::FiveKind,
                _ => unreachable!(),
            },
            2 => match joker_count {
                3 => return HandKind::FiveKind,
                2 => return HandKind::FourKind,
                _ => pair_count += 1,
            },
            _ => match joker_count {
                4 => return HandKind::FiveKind,
                3 => return HandKind::FourKind,
                _ => (),
            },
        }
    }

    if trip_count == 1 {
        if pair_count == 1 {
            return HandKind::FullHouse;
        }
        return HandKind::ThreeKind;
    }

    if pair_count == 2 {
        match joker_count {
            1 => return HandKind::FullHouse,
            _ => return HandKind::TwoPair,
        }
    }

    if pair_count == 1 {
        match joker_count {
            1 => return HandKind::ThreeKind,
            _ => return HandKind::OnePair,
        }
    }

    match joker_count {
        0 => HandKind::HighCard,
        1 => HandKind::OnePair,
        2 => HandKind::ThreeKind,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::{get_resource_lines, resource_path};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse1_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let parsed = parse_problem(&lines, false);
        let exp = vec![
            Hand::new([3, 2, 10, 3, 13], 765, HandKind::OnePair),
            Hand::new([10, 5, 5, 11, 5], 684, HandKind::ThreeKind),
            Hand::new([13, 13, 6, 7, 7], 28, HandKind::TwoPair),
            Hand::new([13, 10, 11, 11, 10], 220, HandKind::TwoPair),
            Hand::new([12, 12, 12, 11, 14], 483, HandKind::ThreeKind),
        ];
        assert_eq!(parsed, exp);
    }

    #[test]
    fn hand_kind_test() {
        let hand = parse_hand("12345", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::HighCard);

        let hand = parse_hand("12315", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::OnePair);

        let hand = parse_hand("12312", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::TwoPair);

        let hand = parse_hand("12115", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::ThreeKind);

        let hand = parse_hand("12121", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FullHouse);

        let hand = parse_hand("18888", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FourKind);

        let hand = parse_hand("99999", false);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FiveKind);
    }

    #[test]
    fn hand_ordering_test() {
        let small = parse_hand("12345", false);
        let big = parse_hand("11345", false);
        #[rustfmt::skip] let str = if small < big { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big > small { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");

        let small = parse_hand("12111", false);
        let big = parse_hand("13111", false);
        #[rustfmt::skip] let str = if small < big { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big > small { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");

        let small = parse_hand("11221", false);
        let big = parse_hand("11221", false);
        #[rustfmt::skip] let str = if small == big { "".to_string() } else { format!("{:?} == {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big == small { "".to_string() } else { format!("{:?} == {:?}", small, big) };
        assert_eq!(str, "");
    }

    #[test]
    fn part1_sample_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let mut hands = parse_problem(&lines, false);
        hands.sort();

        let value = hands
            .iter()
            .enumerate()
            .fold(0i32, |acc, (idx, h)| acc + ((idx as i32 + 1) * h.bet));

        assert_eq!(value, 6440);
    }

    #[test]
    fn part1_main_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let mut hands = parse_problem(&lines, false);
        hands.sort();

        let value = hands
            .iter()
            .enumerate()
            .fold(0i32, |acc, (idx, h)| acc + ((idx as i32 + 1) * h.bet));

        assert_eq!(value, 248217452);
    }

    #[test]
    fn hand_kind_joker_test() {
        let hand = parse_hand("1234J", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::OnePair);

        let hand = parse_hand("123JJ", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::ThreeKind);

        let hand = parse_hand("122JJ", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FourKind);

        let hand = parse_hand("J22JJ", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FiveKind);

        let hand = parse_hand("JJJJJ", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FiveKind);

        let hand = parse_hand("2211J", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FullHouse);

        let hand = parse_hand("88388", true);
        assert_eq!(get_hand_kind(&hand.cards), HandKind::FourKind);
    }

    #[test]
    fn hand_ordering_joker_test() {
        let small = parse_hand("J2345", true);
        let big = parse_hand("22345", true);
        #[rustfmt::skip] let str = if small < big { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big > small { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");

        let small = parse_hand("12111", true);
        let big = parse_hand("13J11", true);
        #[rustfmt::skip] let str = if small < big { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big > small { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");

        let small = parse_hand("1J111", true);
        let big = parse_hand("11111", true);
        #[rustfmt::skip] let str = if small < big { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big > small { "".to_string() } else { format!("{:?} !< {:?}", small, big) };
        assert_eq!(str, "");

        let small = parse_hand("11221", true);
        let big = parse_hand("11221", true);
        #[rustfmt::skip] let str = if small == big { "".to_string() } else { format!("{:?} == {:?}", small, big) };
        assert_eq!(str, "");
        #[rustfmt::skip] let str = if big == small { "".to_string() } else { format!("{:?} == {:?}", small, big) };
        assert_eq!(str, "");
    }

    #[test]
    fn part2_sample_test() {
        let lines = get_resource_lines(&resource_path!("sample.txt").unwrap());
        let mut hands = parse_problem(&lines, true);
        hands.sort();

        let value = hands
            .iter()
            .enumerate()
            .fold(0i32, |acc, (idx, h)| acc + ((idx as i32 + 1) * h.bet));

        assert_eq!(value, 5905);
    }

    #[test]
    fn part2_main_test() {
        let lines = get_resource_lines(&resource_path!("main.txt").unwrap());
        let mut hands = parse_problem(&lines, true);
        hands.sort();

        let value = hands
            .iter()
            .enumerate()
            .fold(0i32, |acc, (idx, h)| acc + ((idx as i32 + 1) * h.bet));

        assert_eq!(value, 245576185);
    }
}
