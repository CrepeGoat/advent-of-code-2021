use aoc_lib::grid::{adj8_coords, read_input, Grid};

use itertools::Itertools;

fn advance_epoch<const ROW: usize, const COL: usize>(
    mut mat: Grid<u32, ROW, COL>,
) -> Grid<u32, ROW, COL> {
    const FLASH_THRESHOLD: u32 = 10;
    for (i, j) in (0..ROW).flat_map(|i| (0..COL).map(move |j| (i, j))) {
        mat[i][j] += 1;
    }

    let mut flash_stack: Vec<_> = (0..ROW)
        .flat_map(|i| (0..COL).map(move |j| (i, j)))
        .filter_map(|(i, j)| (mat[i][j] == FLASH_THRESHOLD).then(|| (i, j)))
        .collect();
    while let Some(coord) = flash_stack.pop() {
        for (i, j) in adj8_coords::<ROW, COL>(coord) {
            if mat[i][j] < FLASH_THRESHOLD {
                mat[i][j] += 1;
                if mat[i][j] == FLASH_THRESHOLD {
                    flash_stack.push((i, j));
                }
            }
        }
    }

    for (i, j) in (0..ROW).flat_map(|i| (0..COL).map(move |j| (i, j))) {
        if mat[i][j] == FLASH_THRESHOLD {
            mat[i][j] = 0;
        }
    }

    mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dumbo_grid_small_eg1() {
        let dumbo_grid = [
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ];
        let dumbo_grid_next = advance_epoch(dumbo_grid);
        assert_eq!(
            dumbo_grid_next,
            [
                [3, 4, 5, 4, 3],
                [4, 0, 0, 0, 4],
                [5, 0, 0, 0, 5],
                [4, 0, 0, 0, 4],
                [3, 4, 5, 4, 3],
            ],
        );
    }

    #[test]
    fn test_dumbo_grid_small_eg2() {
        let dumbo_grid = [
            [3, 4, 5, 4, 3],
            [4, 0, 0, 0, 4],
            [5, 0, 0, 0, 5],
            [4, 0, 0, 0, 4],
            [3, 4, 5, 4, 3],
        ];
        let dumbo_grid_next = advance_epoch(dumbo_grid);
        assert_eq!(
            dumbo_grid_next,
            [
                [4, 5, 6, 5, 4],
                [5, 1, 1, 1, 5],
                [6, 1, 1, 1, 6],
                [5, 1, 1, 1, 5],
                [4, 5, 6, 5, 4],
            ],
        );
    }
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();

    let mut dumbo_grid = read_input::<_, 10, 10>(stdin.lock()).unwrap();
    println!("epoch 0: {:?}", dumbo_grid);

    let mut score: usize = 0;
    for _ in 0..100 {
        dumbo_grid = advance_epoch(dumbo_grid);
        score += (0..10)
            .cartesian_product(0..10)
            .filter(|&(i, j)| dumbo_grid[i][j] == 0)
            .count();
    }
    println!("epoch 100: {:?}", dumbo_grid);
    println!("epoch 100 score: {:?}", score);
}
