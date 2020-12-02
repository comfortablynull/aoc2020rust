use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::path::Path;
use std::marker::PhantomData;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn must_reader<P: AsRef<Path>>(path: P) -> BufReader<File> {
    let d = format!("{}", path.as_ref().display());
    let file = match File::open(path) {
        Err(e) => panic!("couldn't open file: {} reason: {}", d, e),
        Ok(v) => v
    };
    return BufReader::new(file);
}

pub trait TypeRead<B: BufRead> {
    fn lines_t<T: FromStr>(self) -> TypedLines<T, B>;
}

pub struct TypeReader<B: BufRead> {
    buf: B
}

impl<B: BufRead> TypeReader<B> {
    pub fn new(buf: B) -> TypeReader<B> {
        TypeReader { buf }
    }
}

impl<B: BufRead> TypeRead<B> for TypeReader<B> {
    fn lines_t<T: FromStr>(self) -> TypedLines<T, B> {
        TypedLines { iter: self.buf.lines(), _phantom: PhantomData }
    }
}


pub struct TypedLines<T: FromStr, B: BufRead> {
    iter: Lines<B>,
    _phantom: PhantomData<T>,
}

impl<T: FromStr, B: BufRead> Iterator for TypedLines<T, B> where <T as FromStr>::Err: std::fmt::Display {
    type Item = Result<T, String>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => match v {
                Ok(v) => match v.parse::<T>() {
                    Ok(v) => Some(Ok(v)),
                    Err(err) => Some(Err(format!("{}", err)))
                },
                Err(err) => Some(Err(format!("{}", err)))
            }
            None => None
        }
    }
}