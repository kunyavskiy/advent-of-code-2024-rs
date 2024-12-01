use std::io::BufRead;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}


pub struct TokenRead<'a> {
    iter: Box<dyn Iterator<Item=String> + 'a>
}

impl TokenRead<'_> {
    pub fn new<'a, R : BufRead + 'a>(read: R) -> TokenRead<'a>  {
        TokenRead {
            iter: Box::new(read.lines()
                .map(|line| line.unwrap().split(" ").map(str::to_string).collect::<Vec<_>>())
                .flatten()
                .into_iter()
                .filter(|s| !s.is_empty())
            )
        }
    }
    pub fn next_str(&mut self) -> Option<String> {
        self.iter.next()
    }
    pub fn next_int(&mut self) -> Option<i32> {
        self.next_str().and_then(|s| s.parse().ok())
    }
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
