use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "NN";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1(reader: &mut TokenRead) -> Result<i32> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().flatten().count();
        Ok(answer)
    }

    assert_eq!(-1, part1(&mut TokenRead::new(BufReader::new(TEST.as_bytes())))?);
    println!("Result = {}", time_snippet!(part1(&mut TokenRead::new(BufReader::new(File::open(INPUT_FILE)?)))?));

    println!("\n=== Part 2 ===");

    fn part2(reader: &mut TokenRead) -> Result<i32> {
        Ok(0)
    }

    assert_eq!(-1, part2(&mut TokenRead::new(BufReader::new(TEST.as_bytes())))?);
    println!("Result = {}", time_snippet!(part2(&mut TokenRead::new(BufReader::new(File::open(INPUT_FILE)?)))?));

    Ok(())
}
