use std::str::FromStr;
use tools::{TypeReader, TypeRead};

const FILE: &str = "../day2/input.txt";

struct Policy {
    lower: u16,
    upper: u16,
    char: char,
    pass: String,
}

impl Policy {
    pub fn validate(&self) -> bool {
        let mut count: u16 = 0;
        for c in self.pass.chars() {
            if c != self.char {
                continue;
            }
            count += 1;
            if count > self.upper {
                return false;
            }
        }
        return self.lower <= count && count <= self.upper;
    }

    pub fn validate2(&self) -> bool {
        let chars: Vec<char> = self.pass.chars().collect();
        (*chars.get(self.lower as usize - 1).unwrap() == self.char) ^ (*chars.get(self.upper as usize - 1).unwrap() == self.char)
    }
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = s.trim().split(" ").collect();
        if sections.len() != 3 {
            return Err("Invalid Policy".to_string());
        }
        let range: Vec<u16> = match sections.get(0) {
            Some(v) => match v.split("-").map(|s| s.parse::<u16>()).collect() {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e))
            },
            None => return Err("oh no".to_string())
        };
        Ok(Policy {
            lower: match range.get(0) {
                Some(v) => *v,
                None => return Err("lower not found".to_string())
            },
            upper: match range.get(1) {
                Some(v) => *v,
                None => return Err("upper not found".to_string())
            },
            char: sections.get(1).unwrap().chars().nth(0).unwrap(),
            pass: sections.get(2).unwrap().to_string(),
        })
    }
}

fn main() {
    let r = TypeReader::new(tools::must_reader(FILE));
    let mut valid: u32 = 0;
    let mut valid2: u32 = 0;
    for p in r.lines_t::<Policy>() {
        let pu = p.unwrap();
        if pu.validate() {
            valid += 1;
        }
        if pu.validate2() {
            valid2 += 1;
        }
    }
    println! {"Day 2 Solution 1: {}", valid}
    println! {"Day 2 Solution 2: {}", valid2}
}
