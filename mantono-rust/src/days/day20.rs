use std::{collections::HashMap, fmt::Display};

pub fn first(input: String) -> String {
    let mut occurences: HashMap<u32, usize> = HashMap::with_capacity(256);

    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();

    tiles.iter().for_each(|tile: &Tile| {
        occurences.insert(tile.up, occurences.get(&tile.up).unwrap_or(&0) + 1);
        occurences.insert(tile.down, occurences.get(&tile.down).unwrap_or(&0) + 1);
        occurences.insert(tile.left, occurences.get(&tile.left).unwrap_or(&0) + 1);
        occurences.insert(tile.right, occurences.get(&tile.right).unwrap_or(&0) + 1);
    });

    let corner_hashes: Vec<u32> = occurences
        .iter()
        .filter(|(_, v)| **v == 1)
        .map(|(k, _)| *k)
        .collect();

    let corners: Vec<&Tile> = tiles
        .iter()
        .filter(|tile| is_corner(tile, &corner_hashes))
        .collect();

    for t0 in &mut tiles {
        for t1 in &tiles {
            if t0.can_attch(t1) {
                t0.attach(t1)
            }
        }
    }

    if corners.len() != 4 {
        panic!("Expected 4 corners, got {}", corners.len())
    }

    let result: u64 = corners.iter().map(|tile| tile.id as u64).product();
    result.to_string()
}

fn is_corner(tile: &Tile, corners: &Vec<u32>) -> bool {
    corners.iter().filter(|hash| tile.can_attch(**hash)).count() == 2
}

#[derive(Debug)]
struct Tile {
    id: u16,
    up: u32,
    left: u32,
    down: u32,
    right: u32,
    attached: Vec<u16>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.up, self.right, self.down, self.left
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
        let up: u32 = Self::up(&img);
        let down: u32 = Self::down(&img);
        let left: u32 = Self::left(&img);
        let right: u32 = Self::right(&img);

        Tile {
            id,
            up,
            left,
            down,
            right,
            attached: Vec::with_capacity(4),
        }
    }

    fn up(img: &Vec<&str>) -> u32 {
        bin_str_to_u32(&img.get(0).unwrap().replace("#", "1").replace(".", "0")).unwrap()
    }

    fn down(img: &Vec<&str>) -> u32 {
        bin_str_to_u32(&img.last().unwrap().replace("#", "1").replace(".", "0")).unwrap()
    }

    fn left(img: &Vec<&str>) -> u32 {
        let num: String = img
            .iter()
            .map(|line| line.chars().nth(0).unwrap())
            .map(Self::conv_char)
            .collect::<String>();

        bin_str_to_u32(&num).unwrap()
    }

    fn right(img: &Vec<&str>) -> u32 {
        let num: String = img
            .iter()
            .map(|line| line.chars().last().unwrap())
            .map(Self::conv_char)
            .collect::<String>();

        bin_str_to_u32(&num).unwrap()
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
        let mut sides = vec![
            self.left,
            self.right,
            self.up,
            self.down,
            other.left,
            other.right,
            other.up,
            other.down,
        ];
        sides.sort();
        sides.dedup();
        sides.len() < 8
    }

    pub fn attach(&mut self, other: &Tile) {
        self.attached.push(other.id);
    }
}

fn bin_str_to_u32(binary: &str) -> Result<u32, String> {
    let bin_rev: String = binary.chars().rev().collect();
    let binary = format!("{}{}", binary, bin_rev);
    u32::from_str_radix(&binary, 2).map_err(|_| format!("Unable to convert {} to u32", binary))
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
