use core::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::BufRead;

type Matrix<T> = Vec<Vec<T>>;

fn read_input<R: BufRead, T>(reader: R) -> Result<(Vec<T>, Vec<Matrix<T>>), Box<dyn Error>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: 'static + Error,
{
    let mut lines = reader.lines().peekable();

    let result1 = lines
        .next()
        .ok_or("no lines in input")??
        .split(',')
        .map(T::from_str)
        .collect::<Result<_, _>>()?;

    let mut result2 = Vec::new();
    while lines.next().is_some() && lines.peek().is_some() {
        result2.push(
            (&mut lines)
                .take(5)
                .map(|s| {
                    s.map_err(Box::new)?
                        .trim()
                        .split_whitespace()
                        .map(T::from_str)
                        .map(|r| r.map_err(|e| e.into()))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, Box<dyn Error>>>()?,
        );
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
    BOARDS: Iterator<Item = Matrix<T>>,
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
    println!("items: {:?}\nboards: {:?}", items, boards);

    let score = first_bingo_winner_score::<_, _, _, _, 5, 5>(items.into_iter(), boards.into_iter());
    println!("score: {:?}", score);
}
