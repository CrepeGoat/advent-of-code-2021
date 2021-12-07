use core::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::BufRead;

fn try_into_matrix<T, const ROW: usize, const COL: usize>(
    data: Vec<Vec<T>>,
) -> Result<[[T; COL]; ROW], Box<dyn std::error::Error>> {
    data.into_iter()
        .map(|r| r.try_into())
        .collect::<Result<Vec<_>, _>>()
        .or(Err(Box::new("incorrect row length")))?
        .try_into()
        .or(Err(Box::new("incorrect col length")))
}

fn read_input<R: BufRead>(
    reader: R,
) -> Result<(Vec<usize>, Vec<BingoBoard<usize, 5, 25>>), Box<dyn std::error::Error>> {
    let mut lines = reader.lines().peekable();

    let result1: Vec<usize> = lines
        .next()
        .ok_or("no lines in input")??
        .split(',')
        .map(usize::from_str)
        .collect::<Result<_, _>>()?;

    let mut result2 = Vec::new();
    while lines.next().is_some() && lines.peek().is_some() {
        let board_lines: Vec<_> = (&mut lines)
            .take(5)
            .map(|s| {
                s?.trim()
                    .split_whitespace()
                    .map(usize::from_str)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<_, _>>()??;
        result2.push(read_bingo_board(board_lines.into_iter())?);
    }

    Ok((result1, result2))
}

fn first_bingo_winner_score<ITEMS, BOARDS, T, U, const COL: usize, const SIZE: usize>(
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
                for j in 0..BingoBoard::<T, COL, SIZE>::ROW {
                    vals_lookups.insert(board[(i, j)], (i, j));
                }
            }
            for i in 0..BingoBoard::<T, COL, SIZE>::ROW {
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

fn main() {
    println!("Enter input: ");
    let stdin = std::io::stdin();

    let (items, boards) = read_input(stdin.lock()).unwrap();

    let score = first_bingo_winner_score(items.into_iter(), boards.into_iter());
    println!("score: {:?}", score);
}
