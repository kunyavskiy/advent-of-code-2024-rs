use anyhow::*;
use adv_code_2024::iter_utils::ZipWithNext;
use adv_code_2024::token_read::TokenRead;
use adv_code_2024::wrapper::solve_day;

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    fn input(reader: &mut TokenRead) -> Vec<Vec<i32>> {
        reader.whole_input_vec(|r| r.next_line_ints())
    }

    fn is_inc(v: &Vec<i32>) -> bool {
        v.zip_with_next().all(|(a, b)| a < b)
    }

    fn is_dec(v: &Vec<i32>) -> bool {
        v.zip_with_next().all(|(a, b)| a > b)
    }

    fn is_small_diff(v: &Vec<i32>) -> bool {
        v.zip_with_next().all(|(a, b)| (a - b).abs() <= 3)
    }
    fn is_safe(v: &Vec<i32>) -> bool {
        (is_inc(v) || is_dec(v)) && is_small_diff(v)
    }
    fn is_safe_after_rem(v: &Vec<i32>) -> bool {
        if is_safe(v) {
            return true;
        }
        (0..v.len()).any(|i| {
            let mut cv = v.clone();
            cv.remove(i);
            is_safe(&cv)
        })
    }

    fn part1(reader: &mut TokenRead) -> Option<i32> {
        let data = input(reader);
        Some(data.iter().filter(|&v| is_safe(v)).count() as i32)
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        let data = input(reader);
        Some(data.iter().filter(|&v| is_safe_after_rem(v)).count() as i32)
    }

    solve_day(
        "02",
        TEST,
        Some(2),
        Some(639),
        Some(4),
        Some(674),
        part1,
        part2,
    )
}
