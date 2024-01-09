use crate::common::{file::read_lines_from_file, strings::matches_substr_on_index};

pub fn day1() {
    let res = get_calibration(".\\src\\day1\\input\\part1-whole.txt");
    println!("Res: {}", res);
}

fn get_calibration(file_name: &str) -> u32 {
    match read_lines_from_file(file_name) {
        Ok(lines) => {
            let mut sum:u32= 0;
            for line in lines {
                let line_num = proccess_line(&line);
                sum += line_num;
            }
            sum
        },
        Err(err) => panic!("{}, {}", err.to_string(), file_name)
    }
}

fn proccess_line(line: &str) -> u32 {
    let mut first:Option<u32> = None;
    let mut last:Option<u32> = None;
    let mut index = 0;
    while index < line.len() {
        let (cointais, num, skip) = string_contains_number_on_index(line, index);
        if cointais {
            if first.is_none() {
                first = Some(num);
            }
            last = Some(num);
            index += skip;
        }
        else {
            index +=1;
        }
    }
    let res = (first.unwrap() * 10) + last.unwrap();
    return res;
}

const DIGIT_WORDS: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [&str;10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn string_contains_number_on_index(text: &str, index: usize) -> (bool, u32, usize) {
    let (contains, number, skip) = string_contains_variant_on_index(text, index, DIGITS);
    if contains {
        return (contains, number, 1);
    }
    else {
        let (contains, number, skip) = string_contains_variant_on_index(text, index, DIGIT_WORDS);
        if contains {
            return (contains, number, 1);
        }
        else {
            return (false, 0, 0);
        }
    }
}

fn string_contains_variant_on_index(text: &str, index: usize, variants: [&str;10]) -> (bool, u32, usize) {
    let mut num:u32 = 0;
    for digit_word in variants {
        if matches_substr_on_index(text, digit_word, index) {
            return (true, num, digit_word.len());
        }
        num +=1;
    }
    return (false, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proccess_line() {
        assert_eq!(proccess_line("abc123xyz"), 13);

        assert_eq!(proccess_line("x7y"), 77);

        assert_eq!(proccess_line("ab4raka567dab2ra"), 42);

        assert_eq!(proccess_line("959eight3two"), 92);

        assert_eq!(proccess_line("vseven9"), 79);

        assert_eq!(proccess_line("6four2"), 62);

        assert_eq!(proccess_line("four"), 44);

        assert_eq!(proccess_line("npskfdstpk2knsm"), 22);

        assert_eq!(proccess_line("djnrmpxjbsbpgzvtjkhq6pkkfshx"), 66);
    }

    #[test]
    fn test_part1() {
        assert_eq!(get_calibration(".\\src\\day1\\input\\part1-short.txt"), 148)
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_calibration(".\\src\\day1\\input\\part2-short.txt"), 329);
    }

    #[test]
    fn test_part2_given() {
        assert_eq!(get_calibration(".\\src\\day1\\input\\part2-given.txt"), 281);
    }
}