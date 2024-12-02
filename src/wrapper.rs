use std::fmt::{Debug, Display};
use crate::token_read::TokenRead;

pub fn solve_day<
    T1: Debug + PartialEq + Display,
    T2: Debug + PartialEq + Display,
>(
    day: &str,
    test: &str,
    expected_test_part1: Option<T1>,
    expected_result_part1: Option<T1>,
    expected_test_part2: Option<T2>,
    expected_result_part2: Option<T2>,
    part1: fn(&mut TokenRead) -> Option<T1>,
    part2: fn(&mut TokenRead) -> Option<T2>,
) -> anyhow::Result<()> {

    fn check<T: Debug + PartialEq + Display>(name: &str, r: &mut TokenRead, solver: fn(&mut TokenRead) -> Option<T>, expected: Option<T>) {
        let result = solver(r);
        println!("Result for {} is {:?}", name, result);
        if expected.is_some() {
            assert_eq!(expected, result);
        }
    }
    println!("Advent of Code 2024 - Day {:0>2}", day);
    check("part 1 test", &mut TokenRead::for_test(test), part1, expected_test_part1);
    check("part 1 input", &mut TokenRead::for_day(day), part1, expected_result_part1);
    check("part 2 test", &mut TokenRead::for_test(test), part2, expected_test_part2);
    check("part 2 input", &mut TokenRead::for_day(day), part2, expected_result_part2);
    anyhow::Ok(())
}
