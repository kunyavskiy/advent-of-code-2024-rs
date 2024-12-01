use anyhow::*;
use adv_code_2024::*;

const TEST: &str = "\
";

fn main() -> Result<()> {
    fn part1(reader: &mut TokenRead) -> Option<i32> {
        None
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        None
    }

    solve_day(
        "",
        TEST,
        None,
        None,
        part1,
        part2,
    )
}
