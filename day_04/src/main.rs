use std::{collections::HashMap};
use std::{error::Error, str::FromStr};
use std::fs;
use std::fmt::{Display};
use std::ops::RangeInclusive;

const BYR_VALID_RANGE: RangeInclusive<usize> = 1920..=2002;
const IYR_VALID_RANGE: RangeInclusive<usize> = 2010..=2020;
const EYR_VALID_RANGE: RangeInclusive<usize> = 2020..=2030;
const VALID_CM_HEIGHTS: RangeInclusive<usize> = 150..=193;
const VALID_IN_HEIGHTS: RangeInclusive<usize> = 59..=76;

fn main() -> Result<(), Box<dyn Error>>  {
    let passports = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&passports));

    Ok(())
}

fn part_01 (passports: &Vec<Passport>) -> usize {
    passports.iter().count()
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
        return true // TODO
    }
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
    let input = fs::read_to_string(filename).unwrap();

    Ok(
        input
            .split("\r\n\r\n")
            .map(|id| id.to_string().replace("\r\n", " "))
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
}