fn main() {
    println!("Hello, world!");
}

pub struct WordMathcer {
    word: String,
    index: usize,
}

impl WordMathcer {
    pub fn new(word: &str) -> WordMathcer {
        WordMathcer {
            word: word.to_string(),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0
    }

    // returns true if the char completed a word match
    pub fn next_char(&mut self, c_in: char) -> bool {
        let next = match self.word.chars().nth(self.index) {
            Some(c) => c,
            None => {
                // should be impossible, but this behavior is pretty safe
                self.index = 0;
                return false;
            }
        };

        if next == c_in {
            self.index += 1;
        } else {
            if c_in == self.word.chars().nth(0).unwrap_or('!') {
                self.index = 1
            } else {
                self.index = 0;
            }
        }

        if self.index == self.word.len() {
            self.index = 0;
            return true;
        };
        return false;
    }
}

pub fn fix_line(line: &str) -> String {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut line = line.to_string();
    for (index, word) in words.iter().enumerate() {
        while line.contains(word) {
            line = line.replacen(word, format!("!!!{}!!!", index + 1).as_str(), 1);
        }
        line = line.replace("!!!", word);
    }

    return line;
}

#[derive(PartialEq)]
pub enum Strategy {
    NumbersOnly,
    StateMachines,
    WrapReplace,
}

pub fn get_numbers(lines: &Vec<&str>, strategy: Strategy) -> Vec<i32> {
    let mut word_matchrs = vec![
        WordMathcer::new("one"),
        WordMathcer::new("two"),
        WordMathcer::new("three"),
        WordMathcer::new("four"),
        WordMathcer::new("five"),
        WordMathcer::new("six"),
        WordMathcer::new("seven"),
        WordMathcer::new("eight"),
        WordMathcer::new("nine"),
    ];

    let mut numbers = Vec::<i32>::new();

    for line in lines {
        let line = match strategy {
            Strategy::WrapReplace => fix_line(line),
            _ => line.to_string(),
        };

        for matcher in word_matchrs.iter_mut() {
            matcher.reset();
        }

        let mut first: Option<i32> = None;
        let mut second: Option<i32> = None;

        for c in line.chars() {
            let mut num: Option<i32> = None;

            if c >= '1' && c <= '9' {
                num = Some(c as i32 - 0x30);
            }

            if strategy == Strategy::StateMachines {
                for (index, matcher) in word_matchrs.iter_mut().enumerate() {
                    if matcher.next_char(c) {
                        if let Ok(n) = TryInto::<i32>::try_into(index) {
                            num = Some(n + 1);
                        }
                    }
                }
            }

            match num {
                Some(n) => {
                    if first == None {
                        first = Some(n);
                    } else {
                        second = Some(n);
                    }
                }
                None => {}
            }
        }

        match first {
            Some(f) => match second {
                Some(s) => numbers.push((f * 10) + s),
                None => numbers.push((f * 10) + f),
            },
            None => {}
        }

        // if let Some(last) = numbers.last() {
        //     // println!("{} -> {}", line, last);
        // }
    }

    return numbers;
}

#[cfg(test)]
mod tests {
    use aoc_shared::read_file;
    use aoc_shared::resource_path;
    use pretty_assertions::assert_eq;

    use crate::Strategy;
    use crate::get_numbers;
    use crate::WordMathcer;

    #[test]
    fn test_word_matcher() {
        let mut wm = WordMathcer::new("hello");
        assert!(!wm.next_char('h'));
        assert!(!wm.next_char('e'));
        assert!(!wm.next_char('l'));
        assert!(!wm.next_char('l'));
        assert!(wm.next_char('o'));

        assert!(!wm.next_char('h'));
        assert!(!wm.next_char('x'));
        assert!(!wm.next_char('e'));
        assert!(!wm.next_char('l'));
        assert!(!wm.next_char('l'));
        assert!(!wm.next_char('o'));
    }

    #[test]
    fn test_matcher_one() {
        let mut wm = WordMathcer::new("one");
        assert!(!wm.next_char('o'));
        assert!(!wm.next_char('n'));
        assert!(wm.next_char('e'));

        let numbers_got = get_numbers(&vec!["1nineeight"], Strategy::StateMachines);
        assert_eq!(vec![18], numbers_got);
    }

    #[test]
    fn test_matcher_line() {
        let numbers_got = get_numbers(
            &vec![
                "two1nine",
                "eightwothree",
                "abcone2threexyz",
                "xtwone3four",
                "4nineeightseven2",
                "zoneight234",
                "7pqrstsixteen",
                "eighthree",
                "sevenine",
                "oneight",
                "xtwone3four",
                "three7one7",
                "eightwothree",
                "oooneeone",
                "eight7eight",
            ],
            Strategy::StateMachines,
        );
        assert_eq!(
            vec![29, 83, 13, 24, 42, 14, 76, 83, 79, 18, 24, 37, 83, 11, 88],
            numbers_got
        );
    }

    #[test]
    fn test_get_numbers() {
        let lines_in = vec!["12", "1a2", "a12", "a1a2a", "132", "a1a3a2a"];
        let numbers_got = get_numbers(&lines_in, Strategy::StateMachines);
        assert_eq!(vec![12; 6], numbers_got);

        let lines_in = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let numbers_got = get_numbers(&lines_in, Strategy::StateMachines);
        assert_eq!(vec![11, 22, 33, 44, 55, 66, 77, 88, 99], numbers_got);
    }

    #[test]
    fn test_sample_data() {
        let path = resource_path!("test.txt").unwrap();
        let content = read_file(&path).unwrap();
        let numbers = get_numbers(&content.split("\n").collect(), Strategy::NumbersOnly);
        assert_eq!(vec![12, 38, 15, 77], numbers);
        assert_eq!(142, numbers.iter().sum());
    }

    #[test]
    fn test_sample_data2() {
        let path = resource_path!("test2.txt").unwrap();
        let content = read_file(&path).unwrap();
        let numbers = get_numbers(&content.split("\n").collect(), Strategy::StateMachines);
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], numbers);
        assert_eq!(281, numbers.iter().sum());
    }

    #[test]
    fn test_real_data() {
        let path = resource_path!("input.txt").unwrap();
        let content = read_file(&path).unwrap();
        let numbers = get_numbers(&content.split("\n").collect(), Strategy::NumbersOnly);
        assert_eq!(54561, numbers.iter().sum());
    }

    #[test]
    fn test_real_data2_state_machines() {
        let path = resource_path!("input2.txt").unwrap();
        let content = read_file(&path).unwrap();
        let numbers = get_numbers(&content.split("\n").collect(), Strategy::StateMachines);
        assert_eq!(54076, numbers.iter().sum());
    }
    #[test]
    fn test_real_data2_wrap_replace() {
        let path = resource_path!("input2.txt").unwrap();
        let content = read_file(&path).unwrap();
        let numbers = get_numbers(&content.split("\n").collect(), Strategy::WrapReplace);
        assert_eq!(54076, numbers.iter().sum());
    }

    #[test]
    fn check_state_machines() {
        let path = resource_path!("input2.txt").unwrap();
        let content = read_file(&path).unwrap();
        let lines = content.split("\n").collect();
        let numbers_exp = get_numbers(&lines, Strategy::WrapReplace);
        let numbers_got = get_numbers(&lines, Strategy::StateMachines);

        for (index, exp) in numbers_exp.iter().enumerate() {
            let exp = exp.clone();
            let got = numbers_got[index];
            if got != exp {
                println!("{} - exp: {}, got: {}", lines[index], exp, got);
            }
            assert_eq!(exp, got);
        }
    }
}
