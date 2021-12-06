use std::collections::{HashMap, HashSet};
use std::ops::Index;

pub struct BingoBoard<T, const COL: usize, const SIZE: usize>([T; SIZE]);

impl<T, const COL: usize, const SIZE: usize> BingoBoard<T, COL, SIZE> {
    pub fn row(&self, i: usize) -> &[T] {
        &self.0[(i * COL)..((i + 1) * COL)]
    }

    pub fn row_mut(&mut self, i: usize) -> &[T] {
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

fn first_bingo_winner<ITEMS, BOARDS, T, const COL: usize, const SIZE: usize>(
    items: ITEMS,
    boards: BOARDS,
) -> Result<T, &'static str>
where
    T: Copy + std::hash::Hash + std::cmp::Eq,
    ITEMS: Iterator<Item = T>,
    BOARDS: Iterator<Item = BingoBoard<T, COL, SIZE>>,
    usize: std::ops::Mul<T>,
{
    let board_states: Vec<_> = boards
        .map(|board| {
            let vals_lookups: HashMap<_, _> = HashMap::new();
            let rows_lookups: Vec<HashSet<_>> = Vec::new();
            let cols_lookups: Vec<HashSet<_>> = Vec::new();

            for i in 0..COL {
                for j in 0..(SIZE / COL) {
                    vals_lookups.insert(board[(i, j)], (i, j));
                }
            }
            for i in 0..(SIZE / COL) {
                rows_lookups.push(board.row(i).iter().collect());
            }
            for i in 0..COL {
                cols_lookups.push(board.col_iter(i).collect());
            }

            (vals_lookups, rows_lookups, cols_lookups)
        })
        .collect();

    items
        .flat_map(|item| {
            board_states
                .iter_mut()
                .filter_map(|(vals, rows, cols)| {
                    vals.remove(&item)
                        .and_then(|coords| Some((vals, rows, cols, coords)))
                })
                .map(|(vals, rows, cols, (i, j))| {
                    rows[j].remove(&item);
                    if rows.len() == 0 {
                        return Err(vals.len() * item);
                    }
                    cols[i].remove(&item);
                    if cols.len() == 0 {
                        return Err(vals.len() * item);
                    }

                    Ok(())
                })
        })
        .collect::<Result<_, _>>()
        .transpose()
        .map_err(|_| "no boards won the game")
}
