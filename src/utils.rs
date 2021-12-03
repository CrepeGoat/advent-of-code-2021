use std::io::BufRead;
use std::str::FromStr;


pub fn parsing_input<R: BufRead, T: FromStr>(reader: R) -> impl Iterator<Item = T> {
    reader
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|s| s.parse::<T>().ok())
}
