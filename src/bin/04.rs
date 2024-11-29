use regex::Regex;
use std::str::FromStr;
advent_of_code::solution!(4);

#[derive(Debug)]
enum Height {
    CM(u32),
    IN(u32),
}

impl Height {
    pub(crate) fn validate(&self) -> bool {
        match self {
            Height::CM(h) => *h >= 150 && *h <= 193,
            Height::IN(h) => *h >= 59 && *h <= 76,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHeightError;

impl FromStr for Height {
    type Err = ParseHeightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_suffix("cm") {
            return s.parse().map(Height::CM).map_err(|_| ParseHeightError);
        }

        if let Some(s) = s.strip_suffix("in") {
            return s.parse().map(Height::IN).map_err(|_| ParseHeightError);
        }

        Err(ParseHeightError)
    }
}

#[derive(Debug)]
struct Passport<'a> {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
    country_id: Option<u32>,
}
#[derive(Debug, PartialEq, Eq)]
struct ParsePassportError;

impl<'a> Passport<'a> {
    pub(crate) fn validate(&self) -> bool {
        self.birth_year >= 1920
            && self.birth_year <= 2002
            && self.issue_year >= 2010
            && self.issue_year <= 2020
            && self.expiration_year >= 2020
            && self.expiration_year <= 2030
            && self.height.validate()
            && Regex::new(r"^#[0-9a-f]{6}$")
                .unwrap()
                .is_match(self.hair_color)
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.eye_color)
            && Regex::new(r"^[0-9]{9}$")
                .unwrap()
                .is_match(self.passport_id)
    }

    fn parse(s: &'a str) -> Result<Self, ParsePassportError> {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;

        for split in s.split(&[' ', '\n']) {
            let (key, value) = split.split_once(':').unwrap();

            match key {
                "byr" => birth_year = Some(value.parse().unwrap()),
                "iyr" => issue_year = Some(value.parse().unwrap()),
                "eyr" => expiration_year = Some(value.parse().unwrap()),
                "hgt" => height = value.parse().ok(),
                "hcl" => hair_color = Some(value),
                "ecl" => eye_color = Some(value),
                "pid" => passport_id = Some(value),
                "cid" => country_id = Some(value.parse().unwrap()),
                &_ => {}
            }
        }

        if birth_year.is_none()
            || issue_year.is_none()
            || expiration_year.is_none()
            || height.is_none()
            || hair_color.is_none()
            || eye_color.is_none()
            || passport_id.is_none()
        {
            return Err(ParsePassportError);
        }

        Ok(Passport {
            birth_year: birth_year.unwrap(),
            issue_year: issue_year.unwrap(),
            expiration_year: expiration_year.unwrap(),
            height: height.unwrap(),
            hair_color: hair_color.unwrap(),
            eye_color: eye_color.unwrap(),
            passport_id: passport_id.unwrap(),
            country_id,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .split("\n\n")
        .filter(|passport| {
            passport.contains("byr:")
                && passport.contains("iyr:")
                && passport.contains("eyr:")
                && passport.contains("hgt:")
                && passport.contains("hcl:")
                && passport.contains("ecl:")
                && passport.contains("pid:")
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .trim()
        .split("\n\n")
        .filter_map(|passport| Passport::parse(passport).ok())
        .filter(|passport| passport.validate())
        .count();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
