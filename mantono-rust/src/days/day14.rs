use regex::Regex;
use std::{convert::TryInto, fmt::Display};

pub fn first(input: String) -> String {
    input
        .lines()
        .filter_map(|line| Instr::from(&line))
        .for_each(|i| println!("{}", i));

    "".to_string()
}

pub fn second(input: String) -> String {
    input
}

enum Instr {
    Assign { addr: usize, value: u64 },
    Mask(BitVec),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Assign { addr, value } => write!(f, "mem[{}] = {}", addr, value),
            Instr::Mask(bits) => write!(f, "{}", bits),
        }
    }
}

lazy_static::lazy_static! {
    static ref MEM: Regex = Regex::new(r"\d+").unwrap();
}

impl Instr {
    pub fn from(line: &str) -> Option<Instr> {
        if line.starts_with("mem") {
            let mut parts = MEM.find_iter(line);
            let addr: usize = parts.next()?.as_str().parse().ok()?;
            let value: u64 = parts.next()?.as_str().parse().ok()?;
            Some(Instr::Assign { addr, value })
        } else if line.starts_with("mask") {
            let mask: &str = line.split("=").last()?.trim();
            Some(Instr::Mask(BitVec::from(mask).unwrap()))
        } else {
            None
        }
    }
}

struct BitVec {
    bits: [Bit; 36],
}

impl BitVec {
    pub fn from(mask: &str) -> Result<BitVec, String> {
        if mask.len() != 36 {
            Err(format!("Invalid length for BitVec: {}", mask.len()))
        } else {
            let bits: [Bit; 36] = mask
                .chars()
                .map(|c| Bit::from_char(c).unwrap())
                .collect::<Vec<Bit>>()
                .try_into()
                .unwrap();
            Ok(BitVec { bits })
        }
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.bits {
            write!(f, "{}", b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Bit {
    One,
    Zero,
    Off,
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = match self {
            Bit::One => '1',
            Bit::Zero => '0',
            Bit::Off => 'X',
        };
        write!(f, "{}", c)
    }
}

impl Bit {
    pub fn from_char(c: char) -> Result<Bit, String> {
        match c {
            '1' => Ok(Bit::One),
            '0' => Ok(Bit::Zero),
            'X' => Ok(Bit::Off),
            _ => Err(format!("Invalid char '{}'", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::first;

    #[test]
    fn test_part1() {
        let input = r"
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
        ";

        assert_eq!("165", first(input.to_string()))
    }
}
