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
        .inspect(|(a0, a1)| println!("{} - {}", a1, a0))
        .map(|(a0, a1)| dbg!(a1 - *a0))
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
    println!("{:?}", options);
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

/* fn count_options(adapters: &[usize], current: usize, index: usize) -> usize {
    if adapters.is_empty() {
        return 1;
    }
    //std::thread::sleep(std::time::Duration::from_millis(10));

    adapters
        .iter()
        .take(3)
        .enumerate()
        .inspect(|(i, next)| println!("n: {}", next))
        .filter(|(_, next)| **next - current <= 3)
        .map(|(i, next)| count_options(&adapters[i + 1..], *next, i) + 1)
        .sum()
} */

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
        //            trib(3 + 2) * trib(3 + 1) =
        //            4 * 2 = 8

        // Input:    (0), 1, [4, 5, 6, 7], [10, 11, 12], 15, 16, 19, (22)
        assert_eq!("8", &second(input.to_string()));
    }
}

// [0, 3] => 1
//  R  R
//  trib(3 + 0) = 1

// [0, 1, 3] => 2
//  R  O  R
// trib(3 + 1) => 2
// ---------
// [0, 3]
// [0, 1, 3]
//

// 3 * 2 * 1 * 2 * 1 * 1 =
// 3 * 2 * 2 = 12
// 12 / 3 = 4
// [0, 1, 2, 3] => 4
//  3  2  1
//     1  2  3
//  -  O  O  R
// trib(3 + 2) => 4
// -----------
// [0, 1, 2, 3]
// [0, 1, 3]
// [0, 2, 3]
// [0, 3]

// 3! * 2! * 2! =
// 3 * 2 * 2 * 2 = 24
// 24 / 4 = 6
// [0, 1, 2, 3, 5] => 6
//  3, 2, 2, 1
//     1  2  3  2
// -   O  O  R  R
// --------------
// [0, 1, 2, 3, 5]
// [0, 1, 2, 5]
// [0, 1, 3, 5]
// [0, 2, 3, 5]
// [0, 2, 5]
// [0, 3, 5]

// [0, 1, 2, 3, 6] => 4
//  3  2  1  1
//     1  2  3  1!
//  -  O  O  R  R  => 2^2 = 4
// --------------
// [0, 1, 2, 3, 6]
// [0, 1, 3, 6]
// [0, 2, 3, 6]
// [0, 3, 6]

// [0, 3, 6, 9, 10] => 1
// ---------------
// [0, 3, 6, 9, 10]
//  1, 1, 1, 1, 1

// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22) => 8
//  1   1  3  2  1  1  2   1   1   1   1   1
//  -   R  R  O  O  O  R   O   R   R   R   R    R => 2^3 = 8
// 7

// [0, 1] => 1
//  R  R

// [0, 1, 2] => 2
//  R  O  R

// [0, 1, 2, 3] => 4
//  R  O  O  R
// [1, 2]
// [1]
// [2]
// []

//  [0, 1, 2, 3, 4] => 7
//   3  3  2  1
//      1  2  3  3
//   -  O  O  O  R
// OOO => 7
// 1 2 3
// 1 2
// 1 3
// 2 3
// 1
// 2
// 3
