use core::num;
use std::{iter::Map, collections::HashSet, clone};

use crate::common::file::read_lines_from_file;

pub fn day3() {
    let res = part1(".\\src\\day3\\inputs\\part1-whole.txt");
    println!("Res: {}", res);
}

fn part1(file_name: &str) -> u32 {
    let engine_schema = parse_engine(file_name);
    engine_schema.get_number_part_sum()
}

fn parse_engine(file_name: &str) -> EngineSchema {
    match read_lines_from_file(file_name) {
        Ok(lines) => {
            let engine = find_numbers(&lines);
            engine
        },
        Err(err) => panic!("{}, {}", err.to_string(), file_name)
    }
}

#[derive(Debug)]
struct NumberRecord {
    value: u32,
    x: i32,
    y: i32,
    num_digits: usize
}

impl NumberRecord {
    fn new(value: u32, start: &Coord, num_digits: usize) -> Self {
        NumberRecord {value:value, x:start.x, y: start.y, num_digits }
    }

    fn get_positions_to_check(&self) -> Vec<Coord> {
        let mut positions: Vec<Coord> = Vec::new();
        positions.push(Coord { x: self.x-1, y: self.y-1 });
        positions.push(Coord { x: self.x-1, y: self.y});
        positions.push(Coord { x: self.x-1, y: self.y+1 });

        for i in (0..self.num_digits as i32)  {
            positions.push(Coord { x: self.x + i, y: self.y-1 });
            positions.push(Coord { x: self.x + i, y: self.y+1 });
        }

        positions.push(Coord { x: self.x+(self.num_digits as i32), y: self.y-1 });
        positions.push(Coord { x: self.x+(self.num_digits as i32), y: self.y});
        positions.push(Coord { x: self.x+(self.num_digits as i32), y: self.y+1 });

        positions
    }

    fn is_part_number(&self, symbol_positions: &HashSet<Coord>) -> bool {
        let pos_to_check = self.get_positions_to_check();
        for pos in &pos_to_check {
            if symbol_positions.contains(pos) {
                return true;
            }
        }
        return false;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct EngineSchema {
    numbers: Vec<NumberRecord>,
    symbols: HashSet<Coord>
}

impl EngineSchema {
    fn get_number_part_sum(&self) -> u32 {
        let mut sum:u32 = 0;
        for number in &self.numbers {
            if number.is_part_number(&self.symbols) {
                sum +=  number.value;
            }
        }
        sum
    }
}

fn find_numbers(engine_schema: &Vec<String>) -> EngineSchema {
    let mut numbers: Vec<NumberRecord> = Vec::new();
    let mut symbol_positions: HashSet<Coord> = HashSet::new();
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    let mut parsing_num = false;
    let mut cur_num:u32 = 0;
    let mut cur_num_digits: usize = 0;
    let mut cur_num_start= Coord{x:0,y:0};
    for line in engine_schema {
        for character in line.chars() {
            if parsing_num {
                match character {
                    '.' => {
                        let new_num = NumberRecord::new(cur_num, &cur_num_start, cur_num_digits);
                        numbers.push(new_num);
                        cur_num = 0;
                        cur_num_digits = 0;
                        parsing_num = false;
                    }
                    '0'..='9' => {
                        cur_num = (cur_num * 10) + character.to_digit(10).unwrap();
                        cur_num_digits += 1;
                    }
                    _ => { // special char
                        let new_num = NumberRecord::new(cur_num, &cur_num_start, cur_num_digits);
                        numbers.push(new_num);
                        cur_num = 0;
                        cur_num_digits = 0;
                        symbol_positions.insert(Coord{x:x, y:y});
                        parsing_num = false;
                    }
                }
            }
            else {
                match character {
                    '.' => {} // skip it
                    '0'..='9' => {
                        parsing_num = true;
                        cur_num_start = Coord{x:x, y:y};
                        cur_num_digits = 1;
                        cur_num = character.to_digit(10).unwrap();
                    }
                    _ => {
                        symbol_positions.insert(Coord{x:x, y:y});
                    }
                }
            }
            x +=1;
        }
        x = 0;  
        y += 1;
    }
    EngineSchema{ numbers: numbers, symbols: symbol_positions}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parsing(){
        let engine = parse_engine(".\\src\\day3\\inputs\\part1-example.txt");
        println!("{:?}", engine)
    }

    #[test]
    pub fn test_finding_positions(){
        let engine = parse_engine(".\\src\\day3\\inputs\\part1-example.txt");
        let num = engine.numbers.get(2).unwrap();
        assert_eq!(num.value, 35);
        let poss = num.get_positions_to_check();
        println!("x:{}, y:{}", num.x, num.y);
        println!("{:?}", poss);
        assert_eq!(num.is_part_number(&engine.symbols), true);
    }

    #[test]
    pub fn test_part1(){
        assert_eq!(part1(".\\src\\day3\\inputs\\part1-example.txt"), 4361)
    }
}