use core::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Index;

pub struct BingoBoard<T, const COL: usize, const SIZE: usize>([T; SIZE]);

impl<T, const COL: usize, const SIZE: usize> BingoBoard<T, COL, SIZE> {
    pub fn row(&self, i: usize) -> &[T] {
        &self.0[(i * COL)..((i + 1) * COL)]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.0[(i * COL)..((i + 1) * COL)]
    }

    pub fn col_iter(&self, i: usize) -> impl Iterator<Item = &T> {
        (self.0[i..]).iter().step_by(COL)
    }

    pub fn col_iter_mut(&mut self, i: usize) -> impl Iterator<Item = &mut T> {
        (self.0[i..]).iter_mut().step_by(COL)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T, const COL: usize, const SIZE: usize> Index<(usize, usize)> for BingoBoard<T, COL, SIZE> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < COL);
        assert!(index.1 < SIZE / COL);

        &self.0[index.0 + COL * index.1]
    }
}

fn read_bingo_board<'a, I, T, const COL: usize, const SIZE: usize>(
    mut lines: I,
) -> Result<BingoBoard<T, COL, SIZE>, &'static str>
where
    T: Copy + FromStr,
    I: Iterator<Item = &'a str>,
{
    let mut result: BingoBoard<T, COL, SIZE> = unsafe { std::mem::uninitialized() };

    for i in 0..(COL / SIZE) {
        let board_row = &mut result.row_mut(i);

        let mut items = lines
            .next()
            .ok_or("not enough lines")?
            .trim()
            .split_whitespace()
            .map(|x| T::from_str(x).map_err(|_| "failed to parse bingo board item"));

        for j in 0..COL {
            let board_cell = &mut board_row[j];
            let item = items.next().ok_or("not enough lines")??;

            unsafe {
                std::ptr::write(board_cell as *mut T, item);
            }
        }

        if items.next().is_some() {
            return Err("too many items in line");
        }
    }

    if lines.next().is_some() {
        return Err("too many lines");
    }

    Ok(result)
}

fn first_bingo_winner<ITEMS, BOARDS, T, U, const COL: usize, const SIZE: usize>(
    items: ITEMS,
    boards: BOARDS,
) -> Result<U, &'static str>
where
    T: Copy + std::hash::Hash + std::cmp::Eq + std::ops::Mul<usize, Output = U>,
    ITEMS: Iterator<Item = T>,
    BOARDS: Iterator<Item = BingoBoard<T, COL, SIZE>>,
{
    let mut board_states: Vec<_> = boards
        .map(|board| {
            let mut vals_lookups: HashMap<_, _> = HashMap::new();
            let mut rows_lookups: Vec<HashSet<_>> = Vec::new();
            let mut cols_lookups: Vec<HashSet<_>> = Vec::new();

            for i in 0..COL {
                for j in 0..(SIZE / COL) {
                    vals_lookups.insert(board[(i, j)], (i, j));
                }
            }
            for i in 0..(SIZE / COL) {
                rows_lookups.push(board.row(i).iter().copied().collect());
            }
            for i in 0..COL {
                cols_lookups.push(board.col_iter(i).copied().collect());
            }

            (vals_lookups, rows_lookups, cols_lookups)
        })
        .collect();

    for item in items {
        for (vals, rows, cols) in board_states.iter_mut() {
            if let Some((i, j)) = vals.remove(&item) {
                rows[j].remove(&item);
                if rows.is_empty() {
                    return Ok(item * vals.len());
                }
                cols[i].remove(&item);
                if cols.is_empty() {
                    return Ok(item * vals.len());
                }
            }
        }
    }
    Err("no boards won the game")
}

fn read_input<R: BufRead>(
    reader: R,
) -> Result<(Vec<usize>, Vec<BingoBoard<usize, 5, 25>>), Box<dyn std::error::Error>> {
    let mut lines = reader.lines();

    let result1: Vec<usize> = lines
        .next()
        .ok_or("no lines in input")?
        .split(',')
        .map(usize::from_str)
        .collect()?;

    let result2 = Vec::new();
    while lines.next().is_some() {
        result2.append(read_bingo_board(lines.take(5))?);
    }

    Ok((result1, result2))
}
