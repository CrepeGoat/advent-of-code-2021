use crate::utils::ArrayWrapper;
use std::io::BufRead;

pub type Grid<T, const ROW: usize, const COL: usize> = [[T; COL]; ROW];

pub fn adj_coords<const ROW: usize, const COL: usize>(
    center: (usize, usize),
) -> impl 'static + Iterator<Item = (usize, usize)> {
    let mut result = Vec::new();

    if let Some(offset_0) = center.0.checked_sub(1) {
        result.push((offset_0, center.1));
    }
    if let Some(offset_1) = center.1.checked_sub(1) {
        result.push((center.0, offset_1));
    }
    if center.0 < ROW - 1 {
        result.push((center.0 + 1, center.1));
    }
    if center.1 < COL - 1 {
        result.push((center.0, center.1 + 1));
    }

    result.into_iter()
}

pub fn read_input<R: BufRead, const ROW: usize, const COL: usize>(
    reader: R,
) -> Result<Grid<u32, ROW, COL>, &'static str> {
    reader
        .lines()
        .map(|r| r.map_err(|_| "bad line read"))
        .map(|r| -> Result<_, &'static str> {
            r.and_then(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).ok_or("bad digit"))
                    .collect::<Result<ArrayWrapper<_, COL>, _>>()
                    .map(From::from)
            })
        })
        .collect::<Result<ArrayWrapper<_, ROW>, _>>()
        .map(From::from)
}
