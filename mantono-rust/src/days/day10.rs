fn transform(input: String) -> Vec<usize> {
    let mut adapters: Vec<usize> = input
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    adapters
}

pub fn first(input: String) -> String {
    let adapters: Vec<usize> = transform(input);

    let mut d1 = 0;
    let mut d3 = 0;

    let mut others: Vec<usize> = adapters.clone();
    let device: usize = *others.last().unwrap() + 3;
    others.push(device);
    adapters
        .iter()
        .zip(others)
        .map(|(a0, a1)| a1 - *a0)
        .for_each(|n| match n {
            1 => d1 += 1,
            3 => d3 += 1,
            _ => panic!("NO"),
        });

    (d1 * d3).to_string()
}

pub fn second(input: String) -> String {
    let adapters: Vec<usize> = transform(input);
    let options: Vec<bool> = Vec::with_capacity(adapters.len());

    count_options(&adapters, options).to_string()
}

fn count_options(adapters: &[usize], mut options: Vec<bool>) -> usize {
    if adapters.is_empty() {
        return summarize(options);
    }

    let optional: bool = optional(&adapters);
    options.push(optional);

    count_options(&adapters[1..], options)
}

fn optional(adapters: &[usize]) -> bool {
    let previous: Option<&usize> = adapters.get(0);
    let next: Option<&usize> = adapters.get(2);
    match (previous, next) {
        (None, _) | (_, None) => false,
        (Some(previous), Some(next)) => next - previous <= 3,
    }
}

fn summarize(options: Vec<bool>) -> usize {
    let mut opt_state: usize = 0;
    let mut trib_sums: Vec<usize> = Vec::with_capacity(options.len() / 4);
    for optional in options {
        if optional {
            opt_state += 1;
        } else if opt_state > 0 {
            let trib: usize = trib(2 + opt_state);
            trib_sums.push(trib);
            opt_state = 0;
        }
    }
    trib_sums.iter().product()
}

fn trib(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => trib(n - 3) + trib(n - 2) + trib(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::second;

    #[test]
    fn test_part2() {
        let input = r"
16
10
15
5
1
11
7
19
6
12
4";
        // Input:    (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
        //            R   R  R  O  O  R   R   O   R   R   R   R   R
        //            trib(2 + 2) * trib(2 + 1) =
        //            4 * 2 = 8
        assert_eq!("8", &second(input.to_string()));
    }
}
