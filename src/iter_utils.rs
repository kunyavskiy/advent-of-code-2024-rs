use std::iter::Zip;
use std::slice::Iter;

pub trait ZipWithNext<'a, Iter, T> {
    fn zip_with_next(&'a self) -> Zip<Iter, Iter>;
}

impl <'a, T> ZipWithNext<'a, Iter<'a, T>, T> for Vec<T> {
    fn zip_with_next(&'a self) -> Zip<Iter<'a, T>, Iter<'a, T>> {
        self.iter().zip(self[1..].iter())
    }
}