#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str)-> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err(String::from("error parsing Antigen")),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Negative),
            "+" => Ok(Self::Positive),
            _ => Err(String::from("error parsing RhFactor")),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        let tab = [
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
        for ele in tab {
            let part1 = ele.antigen == self.antigen && ele.rh_factor == self.rh_factor;
            let part2 = ele.antigen == other.antigen && ele.rh_factor == other.rh_factor;

            if part1 && part2 {
                return Ordering::Equal;
            } else if part1 {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        Ordering::Less
    }
}

impl FromStr for BloodType {
     type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen, rh_factor) = s.split_at(s.len() - 1);
        let Ok(antigen) = antigen.parse::<Antigen>() else {
            return Err(String::from("error parsing Antigen"));
        };
        let Ok(rh_factor) = rh_factor.parse::<RhFactor>() else {
            return Err(String::from("error parsing RhFactor"));
        };
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rh_factor {
            RhFactor::Negative => write!(f, "{:?}-", self.antigen),
            RhFactor::Positive => write!(f, "{:?}+", self.antigen),
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }
        match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (_, Antigen::O) => true,
            (Antigen::A, Antigen::A) => true,
            (Antigen::B, Antigen::B) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
         let mut vec = vec![];
        let tab = [
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
          for ele in tab {
            if self.can_receive_from(&ele) {
                vec.push(ele);
            }
        }
        vec
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut vec = vec![];
        let tab = [
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
          for ele in tab {
            if ele.can_receive_from(self) {
                vec.push(ele);
            }
        }
        vec
    }
}