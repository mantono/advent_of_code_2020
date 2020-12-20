use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

pub fn first(input: String) -> String {
    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();
    let mut attached_tiles: Vec<(u16, u16)> = Vec::with_capacity(tiles.len());

    for t0 in &tiles {
        for t1 in &tiles {
            if t1.id != t0.id && t0.can_attch(t1) {
                attached_tiles.push((t0.id, t1.id));
            }
        }
    }

    /* if corners.len() != 4 {
        panic!("Expected 4 corners, got {}", corners.len())
    } */

    attached_tiles
        .iter()
        .group_by(|(k, _)| k)
        .into_iter()
        .map(|(k, group)| (k, group.map(|(_, v0)| *v0).collect::<Vec<u16>>().len()))
        .inspect(|(k, g)| println!("{}, {:?}", k, g))
        .filter(|(_, v)| v <= &2)
        .map(|(k, _)| *k as u64)
        .product::<u64>()
        .to_string()
    //.collect::<Vec<usize>>();
}

#[derive(Debug)]
struct Tile {
    id: u16,
    up: u16,
    left: u16,
    down: u16,
    right: u16,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {}, {}, {}, {}",
            self.id, self.up, self.right, self.down, self.left
        )
    }
}

impl Tile {
    pub fn from(input: &str) -> Tile {
        let mut lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());
        let id: u16 = lines.next().unwrap()[5..9].parse::<u16>().unwrap();
        let img: Vec<&str> = lines.collect();
        let up: u16 = Self::up(&img);
        let down: u16 = Self::down(&img);
        let left: u16 = Self::left(&img);
        let right: u16 = Self::right(&img);

        Tile {
            id,
            up,
            left,
            down,
            right,
        }
    }

    fn up(img: &Vec<&str>) -> u16 {
        bin_str_to_u16(&img.get(0).unwrap().replace("#", "1").replace(".", "0")).unwrap()
    }

    fn down(img: &Vec<&str>) -> u16 {
        bin_str_to_u16(&img.last().unwrap().replace("#", "1").replace(".", "0")).unwrap()
    }

    fn left(img: &Vec<&str>) -> u16 {
        let num: String = img
            .iter()
            .map(|line| line.chars().nth(0).unwrap())
            .map(Self::conv_char)
            .collect::<String>();

        bin_str_to_u16(&num).unwrap()
    }

    fn right(img: &Vec<&str>) -> u16 {
        let num: String = img
            .iter()
            .map(|line| line.chars().last().unwrap())
            .map(Self::conv_char)
            .collect::<String>();

        bin_str_to_u16(&num).unwrap()
    }

    fn conv_char(c: char) -> char {
        match c {
            '#' => '1',
            '.' => '0',
            _ => panic!("Unexpected char '{}'", c),
        }
    }
    /*
    pub fn can_attch(&self, side: u32) -> bool {
        self.down == side || self.right == side || self.down == side || self.left == side
    } */

    pub fn can_attch(&self, other: &Tile) -> bool {
        let regular = vec![other.left, other.right, other.up, other.down];
        let reversed = vec![
            other.left.reverse_bits(),
            other.right.reverse_bits(),
            other.up.reverse_bits(),
            other.down.reverse_bits(),
        ];
        if regular.contains(&self.left) || reversed.contains(&self.left) {
            true
        } else if regular.contains(&self.right) || reversed.contains(&self.right) {
            true
        } else if regular.contains(&self.up) || reversed.contains(&self.up) {
            true
        } else if regular.contains(&self.down) || reversed.contains(&self.down) {
            true
        } else {
            false
        }
    }
}

fn bin_str_to_u16(binary: &str) -> Result<u16, String> {
    u16::from_str_radix(binary, 2).map_err(|_| format!("Unable to convert {} to u16", binary))
}

pub fn second(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use super::first;

    #[test]
    fn test_part1() {
        let input = r"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###

            Tile 1951:
            #.##...##.
            #.####...#
            .....#..##
            #...######
            .##.#....#
            .###.#####
            ###.##.##.
            .###....#.
            ..#.#..#.#
            #...##.#..

            Tile 1171:
            ####...##.
            #..##.#..#
            ##.#..#.#.
            .###.####.
            ..###.####
            .##....##.
            .#...####.
            #.##.####.
            ####..#...
            .....##...

            Tile 1427:
            ###.##.#..
            .#..#.##..
            .#.##.#..#
            #.#.#.##.#
            ....#...##
            ...##..##.
            ...#.#####
            .#.####.#.
            ..#..###.#
            ..##.#..#.

            Tile 1489:
            ##.#.#....
            ..##...#..
            .##..##...
            ..#...#...
            #####...#.
            #..#.#.#.#
            ...#.#.#..
            ##.#...##.
            ..##.##.##
            ###.##.#..

            Tile 2473:
            #....####.
            #..#.##...
            #.##..#...
            ######.#.#
            .#...#.#.#
            .#########
            .###.#..#.
            ########.#
            ##...##.#.
            ..###.#.#.

            Tile 2971:
            ..#.#....#
            #...###...
            #.#.###...
            ##.##..#..
            .#####..##
            .#..####.#
            #..#.#..#.
            ..####.###
            ..#.#.###.
            ...#.#.#.#

            Tile 2729:
            ...#.#.#.#
            ####.#....
            ..#.#.....
            ....#..#.#
            .##..##.#.
            .#.####...
            ####.#.#..
            ##.####...
            ##..#.##..
            #.##...##.

            Tile 3079:
            #.#.#####.
            .#..######
            ..#.......
            ######....
            ####.#..#.
            .#...#.##.
            #.#####.##
            ..#.###...
            ..#.......
            ..#.###...
        ";

        assert_eq!("20899048083289", first(input.to_string()))
    }
}

// 215340, 91752,  237468, 510270
// 727437, 510270, 577713, 861771
// 989583, 294930, 24672,  924039
// 970935, 239964, 215340, 589833
// 555489, 118968, 239964, 989583
// 868395, 579249, 18720,  970935
// 165396, 579249, 87720,  467022
// 589833, 727437, 87720,  278466
// 719349, 270402, 188532, 630873
