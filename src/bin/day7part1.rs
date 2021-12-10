use core::fmt::Debug;
use core::str::FromStr;
use std::io::BufRead;

#[derive(Debug)]
pub enum ParseInputError<T: FromStr>
where
    <T as FromStr>::Err: Debug,
{
    InnerType(<T as FromStr>::Err),
    IO(std::io::Error),
}

pub fn parse_input<R: BufRead, T: FromStr>(mut reader: R) -> Result<Vec<T>, ParseInputError<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).map_err(ParseInputError::IO)?;

    buffer
        .split(',')
        .map(T::from_str)
        .map(|r| r.map_err(ParseInputError::InnerType))
        .collect()
}

fn min_crab_fuel(locs: Vec<usize>) -> usize {
    let loc_counts = locs.iter().fold(std::collections::HashMap::new(), |loc_cntr, &i| {
        let prev_count = loc_cntr.remove(&i).unwrap_or_default();
        loc_cntr.insert(&i, prev_count + 1);
        loc_cntr
    });
    let zero_index_score = locs.iter().map(|&i| )
    let prefix_sums = locs.iter
}
