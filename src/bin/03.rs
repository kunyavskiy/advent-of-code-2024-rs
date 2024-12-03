use anyhow::*;
use regex::Regex;
use adv_code_2024::token_read::TokenRead;
use adv_code_2024::wrapper::solve_day;

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    fn input(reader: &mut TokenRead) -> Vec<String> {
        reader.whole_input_vec(|r| r.next_line())
    }
    
    fn mul(a: &str, b: &str) -> i32 {
        a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    }

    fn solve(reader: &mut TokenRead, does: bool) -> Option<i32> {
        let re = Regex::new("mul\\((\\d\\d?\\d?),(\\d\\d?\\d?)\\)|do\\(\\)|don't\\(\\)").unwrap();
        let data = input(reader);
        let mut r = Vec::new();
        let mut enabled = true;
        for s in data {
            for m in re.captures_iter(s.as_str()) {
                match m.get(0).unwrap().as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _default => {
                        if enabled || !does {
                            r.push(mul(m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str()));
                        }
                    }
                }
            }
        }
        Some(r.iter().sum())
    }

    fn part1(reader: &mut TokenRead) -> Option<i32> {
        solve(reader, false)
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        solve(reader, true)
    }

    solve_day(
        "03",
        TEST,
        Some(161),
        Some(192767529),
        Some(48),
        Some(104083373),
        part1,
        part2,
    )
}
