pub fn day4a() -> String {
    let passports = read_data(false);
    passports
        .iter()
        .filter(|&p| p.is_valid())
        .count()
        .to_string()
}

pub fn day4b() -> String {
    let passports = read_data(true);
    passports
        .iter()
        .filter(|&p| p.is_valid())
        .count()
        .to_string()
}

struct Passport(u8);

impl Passport {
    const BYR: u8 = 1;
    const IYR: u8 = 1 << 1;
    const EYR: u8 = 1 << 2;
    const HGT: u8 = 1 << 3;
    const HCL: u8 = 1 << 4;
    const ECL: u8 = 1 << 5;
    const PID: u8 = 1 << 6;
    const CID: u8 = 1 << 7;

    fn new(s: &str, strict: bool) -> Self {
        let val = s.split_whitespace().fold(0u8, |total, field| {
            let splits = field.split(':').collect::<Vec<&str>>();
            let field_name = splits[0];
            let field_value = splits[1];
            match field_name {
                "byr" => {
                    if !strict || Passport::validate_byr(field_value) {
                        total | Passport::BYR
                    } else {
                        total
                    }
                }
                "iyr" => {
                    if !strict || Passport::validate_iyr(field_value) {
                        total | Passport::IYR
                    } else {
                        total
                    }
                }
                "eyr" => {
                    if !strict || Passport::validate_eyr(field_value) {
                        total | Passport::EYR
                    } else {
                        total
                    }
                }
                "hgt" => {
                    if !strict || Passport::validate_hgt(field_value) {
                        total | Passport::HGT
                    } else {
                        total
                    }
                }
                "hcl" => {
                    if !strict || Passport::validate_hcl(field_value) {
                        total | Passport::HCL
                    } else {
                        total
                    }
                }
                "ecl" => {
                    if !strict || Passport::validate_ecl(field_value) {
                        total | Passport::ECL
                    } else {
                        total
                    }
                }
                "pid" => {
                    if !strict || Passport::validate_pid(field_value) {
                        total | Passport::PID
                    } else {
                        total
                    }
                }
                "cid" => total | Passport::CID,
                _ => total,
            }
        });
        Passport(val)
    }

    fn is_valid(&self) -> bool {
        (self.0 & 127u8) == 127u8
    }

    fn validate_byr(s: &str) -> bool {
        matches!(s.parse::<usize>(), Ok(v) if (1920..=2002).contains(&v))
    }

    fn validate_iyr(s: &str) -> bool {
        matches!(s.parse::<usize>(), Ok(v) if (2010..=2020).contains(&v))
    }

    fn validate_eyr(s: &str) -> bool {
        matches!(s.parse::<usize>(), Ok(v) if (2020..=2030).contains(&v))
    }

    fn validate_hgt(s: &str) -> bool {
        use regex::Regex;
        const REGEX: &str = r"^(\d+)(cm|in)$";
        let re = Regex::new(REGEX).unwrap();
        match re.captures(s) {
            Some(cap) => {
                let height = cap[1].parse::<usize>().unwrap_or(0);
                let unit = &cap[2];
                match unit {
                    "cm" => (150..=193).contains(&height),
                    "in" => (59..=76).contains(&height),
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn validate_hcl(s: &str) -> bool {
        use regex::Regex;
        const REGEX: &str = r"^#([0-9a-f]{6})$";
        let re = Regex::new(REGEX).unwrap();
        re.is_match(s)
    }

    fn validate_ecl(s: &str) -> bool {
        matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }

    fn validate_pid(s: &str) -> bool {
        use regex::Regex;
        const REGEX: &str = r"^\d{9}$";
        let re = Regex::new(REGEX).unwrap();
        re.is_match(s)
    }
}

fn read_data(strict: bool) -> Vec<Passport> {
    use std::fs;
    let values = fs::read_to_string("inputs/day04.txt").expect("Couldn't read file");
    values
        .split("\n\n")
        .map(|s| Passport::new(s, strict))
        .collect::<Vec<Passport>>()
}
