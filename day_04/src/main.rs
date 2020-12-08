use std::{collections::HashMap};
use std::{error::Error, str::FromStr};
use std::fs;
use std::fmt::{Display};
use std::ops::RangeInclusive;
use std::time::Instant;

const BYR_VALID_RANGE: RangeInclusive<usize> = 1920..=2002;
const IYR_VALID_RANGE: RangeInclusive<usize> = 2010..=2020;
const EYR_VALID_RANGE: RangeInclusive<usize> = 2020..=2030;
const VALID_CM_HEIGHTS: RangeInclusive<usize> = 150..=193;
const VALID_IN_HEIGHTS: RangeInclusive<usize> = 59..=76;
const VALID_EYE_COLORS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() -> Result<(), Box<dyn Error>>  {
    let passports = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&passports));
    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();

    println!("Answer 2: {:?}", part_02(&passports));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01 (passports: &Vec<Passport>) -> usize {
    passports.iter().count()
}

fn part_02 (passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|pass| pass.is_valid()).count()
}

#[derive(Debug)]
struct Passport {
    birth_year: String,
    issue_year: String,
    expr_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    pass_id: String,
    country_id: Option<String>
}

enum PassportError {
    MissingField
}

impl Display for PassportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PassportError::MissingField => f.write_str("Missing field")
        }
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        [
            is_number_valid(&self.birth_year, BYR_VALID_RANGE),
            is_number_valid(&self.issue_year, IYR_VALID_RANGE),
            is_number_valid(&self.expr_year, EYR_VALID_RANGE),
            is_height_valid(&self.height),
            is_hair_color_valid(&self.hair_color),
            is_eye_color_valid(&self.eye_color),
            is_pid_valid(&self.pass_id)
        ]
        .iter()
        .all(|&x|  x)
    }
}

fn is_number_valid(number: &String, range: RangeInclusive<usize>) -> bool {
    number.parse::<usize>().ok().map(|n| range.contains(&n)).unwrap_or(false)
}

fn is_height_valid(height: &String) -> bool {
    match height.strip_suffix("cm") {
        Some(rest) => {
            return rest.parse::<usize>().ok()
                .map(|h| VALID_CM_HEIGHTS.contains(&h))
                .unwrap_or(false)
        }
        None => match height.strip_suffix("in") {
            Some(rest) => {
                return rest.parse::<usize>().ok()
                    .map(|h| VALID_IN_HEIGHTS.contains(&h))
                    .unwrap_or(false);
            }
            None => false
        }
    }
}

fn is_hair_color_valid(hair_color: &String) -> bool {
    match hair_color.strip_prefix('#') {
        Some(rest) => u32::from_str_radix(rest, 16).is_ok(),
        None => false
    }
}

fn is_eye_color_valid(eye_color: &String) -> bool {
    VALID_EYE_COLORS.contains(&eye_color.as_str())
}

fn is_pid_valid(passport_id: &String) -> bool {
    passport_id.matches(char::is_numeric).count() == 9
}

impl FromStr for Passport {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mapping: HashMap<String, String> = HashMap::new();
        let fields = s.split_whitespace();

        for field in fields {
            let key_value: Vec<&str> = field.split(':').collect();

            mapping.insert(key_value[0].to_string(), key_value[1].to_string());
        }

        Ok(Passport {
            birth_year: mapping.get("byr").ok_or(PassportError::MissingField)?.to_string(),
            issue_year: mapping.get("iyr").ok_or(PassportError::MissingField)?.to_string(),
            expr_year: mapping.get("eyr").ok_or(PassportError::MissingField)?.to_string(),
            height: mapping.get("hgt").ok_or(PassportError::MissingField)?.to_string(),
            hair_color: mapping.get("hcl").ok_or(PassportError::MissingField)?.to_string(),
            eye_color: mapping.get("ecl").ok_or(PassportError::MissingField)?.to_string(),
            pass_id: mapping.get("pid").ok_or(PassportError::MissingField)?.to_string(),
            country_id: match mapping.get("cid") {
                Some(s) => Some(s.to_string()),
                None => None 
            },
        })
    }

    type Err = PassportError;
}

fn read_file(filename: &str) -> std::io::Result<Vec<Passport>> {
    let input = fs::read_to_string(filename)?;

    Ok(
        input
            .split("\n\n") // \r\n\r\n on Windows
            .map(|id| id.to_string().replace("\n", " ")) // \r\n on Windows
            .filter_map(|id| id.parse().ok())
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let passports = read_file("example.txt").unwrap();
        
        assert_eq!(part_01(&passports), 2);
    }

    #[test]
    fn example_02() {
        let passports = read_file("example_02.txt").unwrap();
        
        assert_eq!(part_02(&passports), 4);
    }
}