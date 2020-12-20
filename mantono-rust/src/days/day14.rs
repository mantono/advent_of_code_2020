use regex::Regex;
use std::{collections::HashMap, convert::TryInto, fmt::Display};

pub fn first(input: String) -> String {
    let mut mem: Memory = Memory::with_capacity(512);

    let instr: Vec<Instr> = input
        .lines()
        .filter_map(|line| Instr::from(&line.trim()))
        .collect();

    for i in instr {
        mem.execute(&i)
    }

    mem.sum().to_string()
}

struct Memory {
    mem: HashMap<usize, u64>,
    mask: [Bit; 36],
}

impl Memory {
    pub fn with_capacity(size: usize) -> Memory {
        Memory {
            mem: HashMap::with_capacity(size),
            mask: [Bit::Off; 36],
        }
    }

    pub fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Assign { addr, value } => self.assign(*addr, *value),
            Instr::Mask { bits } => self.set_mask(bits),
        }
    }

    fn set_mask(&mut self, mask: &[Bit; 36]) {
        self.mask.copy_from_slice(mask);
    }

    fn assign(&mut self, addr: usize, value: u64) {
        let current: u64 = *self.mem.get(&addr).unwrap_or(&0u64);
        self.mem.insert(addr, current + value);
    }

    pub fn sum(&self) -> u64 {
        self.mem.iter().map(|(_, v)| v).sum()
    }
}

pub fn second(input: String) -> String {
    input
}

enum Instr {
    Assign { addr: usize, value: u64 },
    Mask { bits: [Bit; 36] },
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Assign { addr, value } => write!(f, "mem[{}] = {}", addr, value),
            Instr::Mask { bits } => {
                for b in bits.iter() {
                    write!(f, "{}", b)?;
                }
                Ok(())
            }
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
            Some(Instr::bitvec_from(mask).unwrap())
        } else {
            None
        }
    }

    fn bitvec_from(mask: &str) -> Result<Instr, String> {
        if mask.len() != 36 {
            Err(format!("Invalid length for BitVec: {}", mask.len()))
        } else {
            let bits: [Bit; 36] = mask
                .chars()
                .map(|c| Bit::from_char(c).unwrap())
                .collect::<Vec<Bit>>()
                .try_into()
                .unwrap();
            Ok(Instr::Mask { bits })
        }
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
