use std::fmt::{Debug, Display};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub struct TokenRead<'a> {
    iter: Box<dyn Iterator<Item=String> + 'a>
}

impl TokenRead<'_> {
    pub fn new<'a, R : BufRead + 'a>(read: R) -> TokenRead<'a>  {
        TokenRead {
            iter: Box::new(
                read.lines()
                    .flat_map(|line| line.unwrap().split(" ").map(str::to_string).collect::<Vec<_>>())
                    .filter(|s| !s.is_empty())
            )
        }
    }
    pub fn for_test(s: &str) -> TokenRead {
        TokenRead::new(BufReader::new(s.as_bytes()))
    }
    pub fn for_day(day: &str) -> TokenRead {
        let input_file: String = "input/".to_owned() + day + ".txt";
        TokenRead::new(BufReader::new(File::open(input_file).unwrap()))
    }
    pub fn next_str(&mut self) -> Option<String> {
        self.iter.next()
    }
    pub fn next_int(&mut self) -> Option<i32> {
        self.next_str().and_then(|s| s.parse().ok())
    }
    pub fn next_int_pair(&mut self) -> Option<(i32, i32)> {
        let first = self.next_int();
        first.map(|first| { (first, self.next_int().unwrap())})
    }
}

pub fn solve_day<T1, T2>(
    day: &str,
    test: &str,
    expected_part1: Option<T1>,
    expected_part2: Option<T2>,
    part1: fn(&mut TokenRead) -> Option<T1>,
    part2: fn(&mut TokenRead) -> Option<T2>,
) -> Result<()>
where T1: Debug, T1: PartialEq, T1: Display, T2: Debug, T2: Display, T2: PartialEq {
    start_day(day);
    println!("=== Part 1 ===");

    assert_eq!(expected_part1, part1(&mut TokenRead::for_test(test)));
    println!("Result = {:?}", time_snippet!(part1(&mut TokenRead::for_day(day))));

    println!("\n=== Part 2 ===");

    assert_eq!(expected_part2, part2(&mut TokenRead::for_test(test)));
    println!("Result = {:?}", time_snippet!(part2(&mut TokenRead::for_day(day))));

    Ok(())
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
