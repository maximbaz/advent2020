use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day4.txt").expect("Error reading the file")
}

enum Height {
    In(u16),
    Cm(u16),
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Height, ()> {
        s[0..s.len() - 2]
            .parse()
            .map_err(|_| ())
            .and_then(|val| match &s[s.len() - 2..] {
                "cm" => Ok(Height::Cm(val)),
                "in" => Ok(Height::In(val)),
                _ => Err(()),
            })
    }
}

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiry_year: Option<u16>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<EyeColor>,
    passport_id: Option<String>,
}

impl Passport {
    fn has_all_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiry_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        match self.birth_year {
            Some(year) if (1920..=2002).contains(&year) => (),
            _ => return false,
        }

        match self.issue_year {
            Some(year) if (2010..=2020).contains(&year) => (),
            _ => return false,
        }

        match self.expiry_year {
            Some(year) if (2020..=2030).contains(&year) => (),
            _ => return false,
        }

        match self.height {
            Some(Height::Cm(height)) if (150..=193).contains(&height) => (),
            Some(Height::In(height)) if (59..=76).contains(&height) => (),
            _ => return false,
        }

        lazy_static! {
            static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").expect("invalid regex");
            static ref PID: Regex = Regex::new(r"^\d{9}$").expect("invalid regex");
        }

        match &self.hair_color {
            Some(color) if HCL.is_match(color.as_str()) => (),
            _ => return false,
        }

        match &self.passport_id {
            Some(id) if PID.is_match(id.as_str()) => (),
            _ => return false,
        }

        true
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Passport, ()> {
        let fields: HashMap<&str, &str> = s
            .lines()
            .map(|kv| kv.split(':').collect_vec())
            .map(|vec| (vec[0], vec[1]))
            .collect();

        Ok(Passport {
            birth_year: fields.get("byr").and_then(|s| s.parse().ok()),
            issue_year: fields.get("iyr").and_then(|s| s.parse().ok()),
            expiry_year: fields.get("eyr").and_then(|s| s.parse().ok()),
            height: fields.get("hgt").and_then(|s| s.parse().ok()),
            hair_color: fields.get("hcl").and_then(|s| s.parse().ok()),
            eye_color: fields.get("ecl").and_then(|s| s.parse().ok()),
            passport_id: fields.get("pid").and_then(|s| s.parse().ok()),
        })
    }
}

fn input<'a>(string: &'a str) -> Vec<Passport> {
    string
        .trim()
        .split("\n\n")
        .map(|s| s.replace(" ", "\n").trim().parse().expect("invalid input"))
        .collect()
}

fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.has_all_fields()).count()
}

fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(&input(
                &"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
            "
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            0,
            part2(&input(
                &"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
            "
            ))
        );
        assert_eq!(
            4,
            part2(&input(
                &"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
            "
            ))
        );
    }
}
