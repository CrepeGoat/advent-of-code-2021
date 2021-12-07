use core::str::FromStr;
use std::collections::{HashMap, HashSet};

use std::io::BufRead;

fn read_input<R: BufRead, T>(
    reader: R,
) -> Result<(Vec<T>, Vec<Vec<Vec<T>>>), Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let mut lines = reader.lines().peekable();

    let result1 = lines
        .next()
        .ok_or("no lines in input")??
        .split(',')
        .map(T::from_str)
        .collect::<Result<Vec<T>, _>>()?;

    let mut result2 = Vec::new();
    while lines.next().is_some() && lines.peek().is_some() {
        let board_lines: Vec<Vec<T>> = (&mut lines)
            .take(5)
            .map(|s| {
                s.or(Err("bad line read".to_owned()))?
                    .trim()
                    .split_whitespace()
                    .map(T::from_str)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<T>>, _>>()?;
        result2.push(board_lines);
    }

    Ok((result1, result2))
}

fn first_bingo_winner_score<ITEMS, BOARDS, T, U, const ROW: usize, const COL: usize>(
    items: ITEMS,
    boards: BOARDS,
) -> Result<U, &'static str>
where
    T: Copy + std::hash::Hash + std::cmp::Eq + std::ops::Mul<usize, Output = U>,
    ITEMS: Iterator<Item = T>,
    BOARDS: Iterator<Item = Vec<Vec<T>>>,
{
    let mut board_states: Vec<_> = boards
        .map(|board| {
            let mut vals_lookups: HashMap<_, _> = HashMap::new();
            let mut rows_lookups: Vec<HashSet<_>> = Vec::new();
            let mut cols_lookups: Vec<HashSet<_>> = Vec::new();

            for i in 0..ROW {
                for j in 0..COL {
                    vals_lookups.insert(board[i][j], (i, j));
                }
            }
            for i in 0..ROW {
                rows_lookups.push(board[i].iter().copied().collect());
            }
            for j in 0..COL {
                cols_lookups.push(board.iter().map(|row| row[j]).collect());
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

    let (items, boards) = read_input::<_, usize>(stdin.lock()).unwrap();

    let score = first_bingo_winner_score(items.into_iter(), boards.into_iter());
    println!("score: {:?}", score);
}
