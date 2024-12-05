use anyhow::*;
use itertools::Itertools;
use adv_code_2024::token_read::TokenRead;
use adv_code_2024::wrapper::solve_day;

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {

    fn input(reader: &mut TokenRead) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let mut rules: Vec<(i32, i32)> = Vec::new();
        loop {
            let s = reader.next_line().unwrap();
            if s.is_empty() { break }
            let (a, b) = s.split('|').collect_tuple().unwrap();
            rules.push((a.parse().unwrap(), b.parse().unwrap()));
        }
        let things = reader.whole_input_vec(|s| Some(s.next_line()?.split(',').map(|t| t.parse::<i32>().unwrap()).collect_vec()));

        (rules, things)
    }

    fn is_correct_order(rules: &Vec<(i32, i32)>, a: &Vec<i32>) -> bool {
        rules.iter().all(|(l, r)| {
            let i1 = a.iter().position(|x| x == l);
            let i2 = a.iter().position(|x| x == r);
            i1.is_none() || i2.is_none() || i1 < i2
        })
    }
    fn part1(reader: &mut TokenRead) -> Option<i32> {
        let (rules, things) = input(reader);
        Some(
            things
                .iter()
                .filter(|&a| is_correct_order(&rules, a))
                .map(|a| a[a.len() / 2])
                .sum()
        )
    }
    fn fix_order(rules: &Vec<(i32, i32)>, v: &Vec<i32>) -> Vec<i32> {
        let mut g = vec![Vec::new(); v.len()];
        let mut d = vec![0; v.len()];
        for (l, r) in rules.iter() {
            let i1 = v.iter().position(|x| x == l);
            let i2 = v.iter().position(|x| x == r);
            if let (Some(i1), Some(i2)) = (i1, i2) {
                g[i1].push(i2);
                d[i2] += 1
            }
        }
        let mut res = Vec::new();
        for _ in 0..v.len() {
            let p = d.iter().position(|&x| x == 0).unwrap();
            res.push(v[p]);
            d[p] = usize::MAX;
            for &i in g[p].iter() {
                d[i] -= 1
            }
        }
        res
    }
    fn part2(reader: &mut TokenRead) -> Option<i32> {
        let (rules, things) = input(reader);
        Some(
            things
                .iter()
                .filter(|&a| !is_correct_order(&rules, a))
                .map(|a| fix_order(&rules, a))
                .map(|a| a[a.len() / 2])
                .sum()   
        )
    }

    solve_day(
        "05",
        TEST,
        Some(143),
        Some(5329),
        Some(123),
        Some(5833),
        part1,
        part2,
    )
}
