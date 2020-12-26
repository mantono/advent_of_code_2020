use itertools::Itertools;
use lazy_static::lazy_static;
use reduce::Reduce;
use regex::Regex;
use std::{collections::HashSet, fmt::Display, hash::Hash};

lazy_static! {
    static ref WORDS: Regex = Regex::new(r"\w+").unwrap();
}

pub fn first(input: String) -> String {
    let alrg: Vec<Allergen> = transform(&input);
    let all_ingredients: Vec<Vec<&str>> = input.lines().map(words).collect();

    let alrg: Vec<Allergen> = reduce(alrg, 0);
    let not_safe: HashSet<&String> = alrg.iter().filter_map(|x| x.found_in()).collect();
    let safe: Vec<&str> = all_ingredients
        .iter()
        .map(|x| x.iter().filter(|a| !not_safe.contains(&a.to_string())))
        .flatten()
        .map(|x| *x)
        .collect();

    safe.len().to_string()
}

fn words(line: &str) -> Vec<&str> {
    let parts: Vec<&str> = line.split("(contains").collect();
    let first: &str = parts.first().unwrap();
    first.split_whitespace().collect()
}

fn reduce(alrg: Vec<Allergen>, i: usize) -> Vec<Allergen> {
    if alrg.iter().all(|a| a.completed()) {
        return alrg.into_iter().unique().collect();
    }
    let allergen: Allergen = alrg.get(i).unwrap().clone();
    let processed: Vec<Allergen> = alrg.into_iter().map(|a| a.process(&allergen)).collect();
    let next: usize = (i + 1) % processed.len();
    reduce(processed, next)
}

fn transform(input: &str) -> Vec<Allergen> {
    input
        .lines()
        .map(|line| parse_words(line))
        .map(|(ingr, allrg)| create_ingr(allrg, ingr))
        .flatten()
        .collect()
}

fn parse_words(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<&str> = input.trim().split("contains").collect();
    let first: Vec<String> = WORDS
        .find_iter(parts.first().unwrap())
        .map(|w| w.as_str().to_string())
        .collect();
    let second: Vec<String> = WORDS
        .find_iter(parts.last().unwrap())
        .map(|w| w.as_str().to_string())
        .collect();
    (first, second)
}

fn create_ingr(allergens: Vec<String>, ingredients: Vec<String>) -> Vec<Allergen> {
    allergens
        .iter()
        .map(|alg| Allergen::from(alg, &ingredients))
        .collect()
}

#[derive(Clone, Eq)]
struct Allergen {
    pub name: String,
    ingr: HashSet<String>,
}

impl Allergen {
    pub fn from(name: &str, ingr: &Vec<String>) -> Allergen {
        Allergen {
            name: name.to_string(),
            ingr: ingr.into_iter().map(|x| x.to_owned()).collect(),
        }
    }

    pub fn completed(&self) -> bool {
        self.ingr.len() == 1
    }

    pub fn found_in(&self) -> Option<&String> {
        if self.completed() {
            self.ingr.iter().last()
        } else {
            None
        }
    }

    pub fn process(self, other: &Self) -> Self {
        if self.name != other.name {
            if !self.completed() && other.completed() {
                let set: HashSet<String> = subtract(&self.ingr, &other.ingr);
                Allergen { ingr: set, ..self }
            } else {
                self
            }
        } else {
            let intersection: HashSet<String> = intersection(&self.ingr, &other.ingr);
            Allergen {
                ingr: intersection,
                ..self
            }
        }
    }
}

impl PartialEq for Allergen {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ingr == other.ingr
    }
}

impl Hash for Allergen {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
        for x in &self.ingr {
            state.write(x.as_bytes());
        }
    }
}

fn subtract(from: &HashSet<String>, other: &HashSet<String>) -> HashSet<String> {
    let mut from = from.clone();
    for value in other {
        from.remove(value);
    }
    from
}

fn intersection(set0: &HashSet<String>, set1: &HashSet<String>) -> HashSet<String> {
    set0.intersection(set1).map(|x| x.to_owned()).collect()
}

impl Display for Allergen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.ingr)
    }
}

pub fn second(input: String) -> String {
    let alrg: Vec<Allergen> = transform(&input);

    let mut alrg: Vec<Allergen> = reduce(alrg, 0);
    alrg.sort_by(|x, y| x.name.cmp(&y.name));
    for x in &alrg {
        println!("{}", x);
    }
    let not_safe: Vec<&String> = alrg.iter().filter_map(|x| x.found_in()).collect();
    not_safe.iter().join(",").to_string()
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

        /// dairy.1 => mxmxvkd kfcds sqjhc nhms
        /// fish.1 => mxmxvkd kfcds sqjhc nhms
        /// dairy.2 => trh fvjkl sbzzf mxmxvkd
        /// soy.1 => sqjhc fvjkl
        /// fish.2 => sqjhc mxmxvkd sbzzf
        ///
        /// intersection(dairy.1, dairy.2) => mxmxvkd
        /// intersection(fish.1, fish.2) => mxmxvkd sqjhc
        /// mxmxvkd sqjhc - mxmxvkd => sqjhc
        /// sqjhc fvjkl - sqjhc => fvjkl
        ///
        /// dairy = mxmxvkd
        /// fish = sqjhc
        /// soy = fvjkl
        ///
        /// Free: kfcds, nhms, trh, sbzzf
        assert_eq!("5", &first(input.to_string()))
    }
}
