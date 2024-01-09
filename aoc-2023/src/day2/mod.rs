use crate::common::file::read_lines_from_file;

pub fn day2() {
    let res = part1(".\\src\\day2\\inputs\\part1-whole.txt");
    println!("part1 res: {res}");
    let res = part2(".\\src\\day2\\inputs\\part1-whole.txt");
    println!("part2 res: {res}");
}

const AVAILABLE_CUBES: CubeSet = CubeSet{red:12, green:13, blue:14};

fn part1(file_name: &str) -> u32 {
    match read_lines_from_file(file_name) {
        Ok(lines) => {
            let mut sum: u32 = 0;
            for line in &lines {
                let game = parse_game(line);
                if game.is_possible(&AVAILABLE_CUBES) {
                    sum+= game.game_id;
                }
            }
            return sum;
        },
        Err(err) => panic!("{}, {}", err.to_string(), file_name)
    };
}

fn part2(file_name: &str) -> u32 {
    match read_lines_from_file(file_name) {
        Ok(lines) => {
            let mut sum: u32 = 0;
            for line in &lines {
                let game = parse_game(line);
                let power = game.get_required_cube_set().get_power();
                sum += power;
            }
            return sum;
        },
        Err(err) => panic!("{}, {}", err.to_string(), file_name)
    };
}

fn parse_game(line: &str) -> Game {
    let game_and_rest:Vec<&str> = line.split(':').collect();
    let game: Vec<&str> = game_and_rest[0].split_ascii_whitespace().collect();
    let game_num:u32 = game[1].parse().unwrap();

    let cube_set_strs: Vec<&str> = game_and_rest[1].split(';').collect();
    let mut cube_sets: Vec<CubeSet> = Vec::new();
    for cube_set_str  in cube_set_strs {
        let cube_set = parse_cube_set(cube_set_str);
        cube_sets.push(cube_set);
    }
    Game{ game_id: game_num, cube_sets: cube_sets }
}

#[derive(PartialEq, Debug)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32
}

#[derive(PartialEq, Debug)]
struct Game {
    game_id: u32,
    cube_sets: Vec<CubeSet>
}

impl Game {
    fn is_possible(&self, available_cubes: &CubeSet) -> bool {
        for cube_set in &self.cube_sets {
            if cube_set.contains_more_cubes_of_any_color_than(available_cubes) {
                return false;
            }
        }
        return true;
    }

    fn get_required_cube_set(&self) -> CubeSet {
        let mut required = CubeSet::empty();
        for cube_set in &self.cube_sets {
            required.blue = std::cmp::max(required.blue, cube_set.blue);
            required.green = std::cmp::max(required.green, cube_set.green);
            required.red = std::cmp::max(required.red, cube_set.red);
        }
        required
    }
}

impl CubeSet {
    fn empty() -> CubeSet{
        CubeSet{red:0, blue: 0, green:0}
    }

    fn contains_more_cubes_of_any_color_than(&self, other: &CubeSet) -> bool {
        self.blue > other.blue || self.green > other.green || self.red > other.red
    }

    fn get_power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn parse_cube_set(cube_set_str: &str) -> CubeSet {
    let cubes: Vec<&str> = cube_set_str.split(',').collect();
    let mut cube_set = CubeSet::empty();
    for cube in cubes {
        let cube_parts: Vec<&str> = cube.split_ascii_whitespace().collect();
        match cube_parts[1] {
            "red" => {
                cube_set.red = cube_parts[0].parse().unwrap();
            }
            "blue" => {
                cube_set.blue = cube_parts[0].parse().unwrap();
            }
            "green" => {
                cube_set.green = cube_parts[0].parse().unwrap();
            }
            _ => {
                panic!("Unexpected color {}", cube_parts[1]);
             }
        }
    }
    cube_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube_set() {
        assert_eq!(parse_cube_set("1 red, 2 green, 6 blue"), CubeSet{red:1, green:2, blue: 6});
        assert_eq!(parse_cube_set(" 2 green"), CubeSet{red:0, green:2, blue: 0});
        assert_eq!(parse_cube_set(" 3 red, 6 blue"), CubeSet{red:3, green:0, blue: 6});
    }

    #[test]
    fn test_parse_game() {
        let g1 = Game{game_id: 1, cube_sets: vec![
            CubeSet{red:4, green:0, blue:3},
            CubeSet{red:1, green:2, blue:6},
            CubeSet{red:0, green:2, blue:0}]};
        assert_eq!(parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), g1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(".\\src\\day2\\inputs\\part1-example.txt"), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(".\\src\\day2\\inputs\\part1-example.txt"), 2286);
    }
}