use std::collections::HashMap;

pub fn first(input: String) -> String {
    let mut occurences: HashMap<u16, usize> = HashMap::with_capacity(256);

    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();

    tiles.iter().for_each(|tile: &Tile| {
        occurences.insert(tile.up, occurences.get(&tile.up).unwrap_or(&0) + 1);
        occurences.insert(tile.down, occurences.get(&tile.down).unwrap_or(&0) + 1);
        occurences.insert(tile.left, occurences.get(&tile.left).unwrap_or(&0) + 1);
        occurences.insert(tile.right, occurences.get(&tile.right).unwrap_or(&0) + 1);
    });

    let corner_hashes: Vec<u16> = occurences
        .iter()
        .filter(|(_, v)| **v < 3)
        .map(|(k, v)| *k)
        .collect();

    let corners: Vec<&Tile> = tiles
        .iter()
        .filter(|tile| is_corner(tile, &corner_hashes))
        .collect();

    if corners.len() != 4 {
        panic!("Expected 4 corners, got {}", corners.len())
    }

    let result: u64 = corners.iter().map(|tile| tile.id as u64).product();
    result.to_string()
}

fn is_corner(tile: &Tile, corners: &Vec<u16>) -> bool {
    corners.iter().filter(|hash| tile.can_attch(**hash)).count() == 2
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    id: u16,
    up: u16,
    left: u16,
    down: u16,
    right: u16,
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
            .map(|line| line.chars().nth(0).unwrap())
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

    pub fn can_attch(&self, side: u16) -> bool {
        self.up == side || self.right == side || self.down == side || self.left == side
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
