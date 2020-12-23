use itertools::{GroupBy, Itertools};
use lazy_static::lazy_static;
use reduce::Reduce;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    iter::Map,
};

pub fn first(input: String) -> String {
    let mut joined: HashSet<Ingredient> = HashSet::with_capacity(256);

    let mut allergens: Vec<Ingredient> = input
        .lines()
        .map(|line| Ingredient::from(line))
        .flatten()
        .collect();

    allergens.sort_by(|a0, a1| a0.name.cmp(&a1.name));

    let ingredients: Vec<Ingredient> = allergens
        .clone()
        .into_iter()
        .group_by(|alg: &Ingredient| alg.name.clone())
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<Ingredient>>())
        .filter_map(reduce)
        .inspect(|m| println!("{:?}", m))
        .collect();

    "".to_string()
}
/*
fn add(cont: &mut HashMap<String, HashMap<String, usize>>, m: &Ingredient) {
    for key in m.keys {
        let ingr: &mut HashMap<String, usize> = cont
            .get_mut(&key)
            .unwrap_or(&mut HashMap::with_capacity(16));

        ing
    }
} */

fn reduce(mut vec: Vec<Ingredient>) -> Option<Ingredient> {
    vec.sort_by(|a0, a1| a0.name.cmp(&a1.name));
    let reduced: Option<Ingredient> = dbg!(vec)
        .into_iter()
        .reduce(|a0: Ingredient, a1: Ingredient| a0.merge(&a1));

    dbg!(reduced)
}

#[derive(Debug, Clone)]
struct Ingredient {
    pub name: String,
    pub allergens: HashSet<String>,
}

lazy_static! {
    static ref ALG: Regex = Regex::new(r"\w+").unwrap();
}

impl Ingredient {
    pub fn from(line: &str) -> Vec<Ingredient> {
        if line.trim().is_empty() {
            return Vec::with_capacity(0);
        }

        let parts: Vec<&str> = line.trim().split("(contains").collect();

        let allergens: HashSet<String> = ALG
            .find_iter(parts.last().unwrap())
            .map(|s| s.as_str().to_string())
            .collect();

        parts
            .first()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .map(|s| Ingredient {
                name: s,
                allergens: allergens.clone(),
            })
            .collect()
    }

    pub fn merge(self, other: &Self) -> Self {
        if self.name != other.name {
            self
        } else {
            let intersection: HashSet<String> = self
                .allergens
                .intersection(&other.allergens)
                .map(|s| s.to_string())
                .collect();

            Ingredient {
                allergens: intersection,
                ..self
            }
        }
    }
}

pub fn second(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::first;

    #[test]
    fn test_part1() {
        let input = r"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
        ";

        assert_eq!("5", &first(input.to_string()))
    }
}
