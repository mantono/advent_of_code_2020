use regex::Regex;
use std::{collections::HashMap, fmt::Display};

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
    mask_or: u64,
    mask_nand: u64,
}

impl Memory {
    pub fn with_capacity(size: usize) -> Memory {
        Memory {
            mem: HashMap::with_capacity(size),
            mask_or: 0,
            mask_nand: 0,
        }
    }

    pub fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Assign { addr, value } => self.assign(*addr, *value),
            Instr::Mask { or, nand } => self.set_mask(*or, *nand),
        }
    }

    fn set_mask(&mut self, or: u64, nand: u64) {
        self.mask_or = or;
        self.mask_nand = nand;
    }

    fn assign(&mut self, addr: usize, value: u64) {
        let val: u64 = (value | self.mask_or) & !self.mask_nand;
        self.mem.insert(addr, val);
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
    Mask { or: u64, nand: u64 },
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Assign { addr, value } => write!(f, "mem[{}] = {}", addr, value),
            Instr::Mask { or, nand } => write!(f, "{}/{}", or, nand),
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
            Some(Instr::mask_from(mask).unwrap())
        } else {
            None
        }
    }

    fn mask_from(mask: &str) -> Result<Instr, String> {
        if mask.len() != 36 {
            Err(format!("Invalid length for BitVec: {}", mask.len()))
        } else {
            let or: u64 = bin_str_to_u64(&mask.replace("X", "0"))?;
            let nand: String = mask
                .replace("1", "_")
                .replace("0", "1")
                .replace("X", "0")
                .replace("_", "0");
            let nand: u64 = bin_str_to_u64(&nand)?;
            Ok(Instr::Mask { or, nand })
        }
    }
}

fn bin_str_to_u64(binary: &str) -> Result<u64, String> {
    u64::from_str_radix(binary, 2).map_err(|_| format!("Unable to convert {} to u64", binary))
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
