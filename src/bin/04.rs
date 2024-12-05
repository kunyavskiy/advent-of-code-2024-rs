use anyhow::*;
use adv_code_2024::token_read::TokenRead;
use adv_code_2024::wrapper::solve_day;

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    fn input(reader: &mut TokenRead) -> Vec<String> {
        reader.whole_input_vec(|r| r.next_line())
    }

    fn part1(reader: &mut TokenRead) -> Option<i32> {
        let data = input(reader);
        let mut ans = 0;
        for i in 0..data.len() as i32 {
            for j in 0..data[i as usize].len() as i32 {
                for di in -1..2 {
                    for dj in -1..2 {
                        if (i + di * 3 < 0 || i + di * 3 >= data.len() as i32) || (j + dj * 3 < 0 || j + dj * 3 >= data[i as usize].len() as i32) {
                            continue
                        }
                        if (0..4).all(|d| data[(i + di * d) as usize].as_bytes()[(j + dj * d) as usize] == "XMAS".as_bytes()[d as usize]) {
                            ans += 1
                        }
                    }
                }
            }
        }
        Some(ans)
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        let data = input(reader);
        let mut ans = 0;
        for i in 1..data.len() - 1 {
            for j in 1..data[i].len() - 1 {
                if data[i].as_bytes()[j] as char != 'A' { continue; }
                let v = vec![
                    data[i-1].as_bytes()[j-1],
                    data[i-1].as_bytes()[j+1],
                    data[i+1].as_bytes()[j+1],
                    data[i+1].as_bytes()[j-1], 
                ];
                let M = 'M' as u8;
                let S = 'S' as u8;
                if v[0] == M && v[1] == M && v[2] == S && v[3] == S { ans += 1 }
                if v[0] == M && v[1] == S && v[2] == S && v[3] == M { ans += 1 }
                if v[0] == S && v[1] == S && v[2] == M && v[3] == M { ans += 1 }
                if v[0] == S && v[1] == M && v[2] == M && v[3] == S { ans += 1 }
            }
        }
        Some(ans)    }

    solve_day(
        "04",
        TEST,
        Some(18),
        Some(2685),
        Some(9),
        None,
        part1,
        part2,
    )
}
