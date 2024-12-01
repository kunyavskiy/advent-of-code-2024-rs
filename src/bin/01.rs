use anyhow::*;
use adv_code_2024::*;

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    fn input(reader: &mut TokenRead) -> (Vec<i32>, Vec<i32>) {
        let mut a = Vec::new();
        let mut b = Vec::new();
        loop {
            match reader.next_int_pair() {
                None => break,
                Some((x, y)) => {
                    a.push(x);
                    b.push(y);
                }
            }
        }
        (a, b)
    }

    fn part1(reader: &mut TokenRead) -> Option<i32> {
        let (mut a, mut b) = input(reader);
        a.sort();
        b.sort();
        Some(a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum())
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        let (a, b) = input(reader);
        Some(a.iter().map(|&x| b.iter().filter(|&&a| a == x).count() as i32 * x).sum())
    }

    solve_day(
        "01",
        TEST,
        Some(11),
        Some(31),
        part1,
        part2,
    )
}
