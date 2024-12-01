use anyhow::*;
use std::fs::File;
use std::io::BufReader;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn input(reader: &mut TokenRead) -> (Vec<i32>, Vec<i32>) {
        let mut a = Vec::new();
        let mut b = Vec::new();
        loop {
            let x = reader.next_int();
            if x.is_none() {
                break
            }
            a.push(x.unwrap());
            b.push(reader.next_int().unwrap());
        }
        (a, b)
    }

    fn part1(reader: &mut TokenRead) -> Result<i32> {
        let (mut a, mut b) = input(reader);
        a.sort();
        b.sort();
        Ok(a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum())
    }

    assert_eq!(11, part1(&mut TokenRead::new(BufReader::new(TEST.as_bytes())))?);
    println!("Result = {}", time_snippet!(part1(&mut TokenRead::new(BufReader::new(File::open(INPUT_FILE)?)))?));

    println!("\n=== Part 2 ===");

    fn part2(reader: &mut TokenRead) -> Result<i32> {
        let (a, b) = input(reader);
        Ok(a.iter().map(|&x| b.iter().filter(|&&a| a == x).count() as i32 * x).sum())
    }

    assert_eq!(31, part2(&mut TokenRead::new(BufReader::new(TEST.as_bytes())))?);
    println!("Result = {}", time_snippet!(part2(&mut TokenRead::new(BufReader::new(File::open(INPUT_FILE)?)))?));

    Ok(())
}
