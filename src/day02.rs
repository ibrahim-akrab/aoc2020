use regex::Regex;

pub fn day2a() -> String {
    let values = read_data();
    values
        .iter()
        .fold(0usize, |total, p| {
            if p.is_valid_at_sled() {
                total + 1usize
            } else {
                total
            }
        })
        .to_string()
}

pub fn day2b() -> String {
    let values = read_data();
    values
        .iter()
        .fold(0usize, |total, p| {
            if p.is_valid_at_toboggan() {
                total + 1usize
            } else {
                total
            }
        })
        .to_string()
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl PasswordPolicy {
    fn new(s: &str, re: &Regex) -> Option<Self> {
        let matches = re.captures(s)?;
        let min = matches
            .get(1)
            .and_then(|s| s.as_str().parse::<usize>().ok())?;
        let max = matches
            .get(2)
            .and_then(|s| s.as_str().parse::<usize>().ok())?;
        let letter = matches.get(3)?.as_str().to_owned();
        let password = matches.get(4)?.as_str().to_owned();

        Some(Self {
            min,
            max,
            letter,
            password,
        })
    }

    fn is_valid_at_sled(&self) -> bool {
        let count = self.password.matches(&self.letter).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_at_toboggan(&self) -> bool {
        if self.password.len() < self.max {
            return false;
        }
        if self.password.as_bytes()[self.min - 1] == self.password.as_bytes()[self.max - 1] {
            return false;
        }
        if self.password.as_bytes()[self.min - 1] == self.letter.as_bytes()[0] {
            return true;
        }
        if self.password.as_bytes()[self.max - 1] == self.letter.as_bytes()[0] {
            return true;
        }
        false
    }
}

fn read_data() -> Vec<PasswordPolicy> {
    use std::fs;
    let values = fs::read_to_string("inputs/day02.txt").expect("Couldn't read file");
    const REGEX: &str = r"^(\d+)-(\d+) (.): (.*)$";
    let regex = Regex::new(REGEX).unwrap();
    values
        .split('\n')
        .filter_map(|s| PasswordPolicy::new(s, &regex))
        .collect()
}
