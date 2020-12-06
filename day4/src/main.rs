use std::collections::HashMap;

#[derive(Debug)]
struct ParsedPassportData {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    pid: String,
    cid: Option<String>,
}

impl ParsedPassportData {
    fn new(source: &mut HashMap<String, String>) -> Option<Self> {
        Some(ParsedPassportData {
            birth_year: source.remove("byr")?,
            issue_year: source.remove("iyr")?,
            expiration_year: source.remove("eyr")?,
            height: source.remove("hgt")?,
            hair_color: source.remove("hcl")?,
            eye_color: source.remove("ecl")?,
            pid: source.remove("pid")?,
            cid: source.remove("cid"),
        })
    }

    fn contains_valid_data(&self) -> bool {
        valid_birth_year(&self.birth_year)
            && valid_expiration_year(&self.expiration_year)
            && valid_issue_year(&self.issue_year)
            && valid_eye_color(&self.eye_color)
            && valid_hair_color(&self.hair_color)
            && valid_height(&self.height)
            && valid_pid(&self.pid)
    }
}

fn valid_pid(pid: &str) -> bool {
    pid.chars().filter(|x| x.is_numeric()).count() == 9
}

fn valid_eye_color(eye_color: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color)
}

fn valid_hair_color(hair_color: &str) -> bool {
    hair_color
        .strip_prefix('#')
        .map(|hc| hc.chars().filter(char::is_ascii_hexdigit).count() == 6)
        .unwrap_or(false)
}

fn valid_height(height: &str) -> bool {
    match height {
        x if x.ends_with("cm") => x[..x.len() - 2]
            .parse::<u8>()
            .map(|height| (150..=193).contains(&height))
            .unwrap_or(false),

        x if x.ends_with("in") => x[..x.len() - 2]
            .parse::<u8>()
            .map(|height| (59..=76).contains(&height))
            .unwrap_or(false),
        _ => false,
    }
}

fn valid_issue_year(issue_year: &str) -> bool {
    issue_year
        .parse::<u16>()
        .map(|issue_year| (2010..=2020).contains(&issue_year))
        .unwrap_or(false)
}

fn valid_expiration_year(expiration_year: &str) -> bool {
    expiration_year
        .parse::<u16>()
        .map(|expiration_year| (2020..=2030).contains(&expiration_year))
        .unwrap_or(false)
}

fn valid_birth_year(birth_year: &str) -> bool {
    birth_year
        .parse::<u16>()
        .map(|birth_year| (1920..=2002).contains(&birth_year))
        .unwrap_or(false)
}

fn main() {
    let valid_passports = include_str!("input")
        .split(char::is_whitespace)
        .fold(
            (HashMap::new(), vec![]),
            |(mut data, mut parsed_passports), line| {
                if line.is_empty() {
                    if let Some(passport) = ParsedPassportData::new(&mut data) {
                        parsed_passports.push(passport)
                    }

                    data.clear()
                } else {
                    for l in line.split(' ') {
                        match l.split(':').collect::<Vec<&str>>().as_slice() {
                            [key, value] => data.insert(key.to_string(), value.to_string()),
                            _ => unreachable!(),
                        };
                    }
                }

                (data, parsed_passports)
            },
        )
        .1;

    println!("{:?}", valid_passports.iter().count());
    // println!("{:?}", valid_passports);
    println!(
        "{:?}",
        valid_passports
            .iter()
            .filter(|x| x.contains_valid_data())
            .count()
    );
}
