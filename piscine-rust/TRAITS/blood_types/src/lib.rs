#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl Display for Antigen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen.cmp(&other.antigen) == Ordering::Equal {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_at(s.len() - 1);
        let antigen = Antigen::from_str(parts.0)?;
        let rh_factor = RhFactor::from_str(parts.1)?;
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug, Display};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        (match self.antigen {
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
            Antigen::O => matches!(other.antigen, Antigen::O),
        }) && match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut list_donor: Vec<BloodType> = Vec::new();
        if self.antigen == Antigen::O && self.rh_factor == RhFactor::Negative {
            let donor = BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            };
            list_donor.push(donor);
            return list_donor;
        }
        for antigen in &[Antigen::O, Antigen::A, Antigen::B, Antigen::AB] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if self.can_receive_from(&donor) {
                    list_donor.push(donor);
                }
            }
        }
        list_donor
    }
    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recive: Vec<BloodType> = Vec::new();
        for antigen in &[Antigen::O, Antigen::A, Antigen::B, Antigen::AB] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if donor.can_receive_from(&self) {
                    recive.push(donor);
                }
            }
        }
        recive
    }
}
