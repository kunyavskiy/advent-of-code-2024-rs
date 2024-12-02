use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct TokenRead<'a> {
    line_iter: Box<dyn Iterator<Item = String> + 'a>,
    iter: Box<dyn Iterator<Item=String> + 'a>
}

impl TokenRead<'_> {
    pub fn new<'a, R : BufRead + 'a>(read: R) -> TokenRead<'a>  {
        TokenRead {
            line_iter: Box::new(read.lines().map(|s| s.unwrap().to_owned())),
            iter: Box::new(std::iter::empty())
        }
    }
    pub fn for_test(s: &str) -> TokenRead {
        TokenRead::new(BufReader::new(s.as_bytes()))
    }
    pub fn for_day(day: &str) -> TokenRead {
        let input_file: String = "input/".to_owned() + day + ".txt";
        TokenRead::new(BufReader::new(File::open(input_file).unwrap()))
    }

    pub fn next_line_strings(&mut self) -> Option<Vec<String>> {
        let mut result = vec![self.next_str()?];
        loop {
            match self.iter.next() {
                Some(y) => result.push(y),
                None => break
            }
        }
        Some(result)
    }
    pub fn next_line_ints(&mut self) -> Option<Vec<i32>> {
        let strings = self.next_line_strings()?;
        Some(strings.iter().map(|s| s.parse::<i32>().unwrap()).collect())
    }
    pub fn next_str(&mut self) -> Option<String> {
        let mut res = self.iter.next();
        while res == None {
            self.iter = Box::new(self.line_iter.next()?.split(" ").filter(|s| !s.is_empty()).map(str::to_owned).collect::<Vec<_>>().into_iter());
            res = self.iter.next();
        }
        Some(res?.to_owned())
    }
    pub fn next_int(&mut self) -> Option<i32> {
        self.next_str().and_then(|s| s.parse().ok())
    }
    pub fn next_int_pair(&mut self) -> Option<(i32, i32)> {
        let first = self.next_int();
        first.map(|first| { (first, self.next_int().unwrap())})
    }

    pub fn whole_input_vec<R>(&mut self, f: fn (&mut TokenRead) -> Option<R>) -> Vec<R> {
        let mut result = Vec::new();
        loop {
            match f(self) {
                None => return result,
                Some(x) => { result.push(x) }
            }
        }
    }
}