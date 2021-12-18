use aoc_lib::grid::{read_input, Grid};
use core::cmp::min;

fn path_risk<const ROW: usize, const COL: usize>(grid: &Grid<u32, ROW, COL>) -> u32 {
    assert!(COL > 0 && ROW > 0, "zero-sized array");

    let mut init_row = [u32::MAX; COL];
    init_row[0] = 0;

    let last_row_risks = grid.iter().fold(init_row, |col_buffer, row| {
        let mut result_buffer: [u32; COL] = unsafe { std::mem::uninitialized() };

        let mut last_cell = u32::MAX;
        for ((cell, last_best), row_item) in result_buffer
            .iter_mut()
            .zip(col_buffer.iter().copied())
            .zip(row.iter().copied())
        {
            unsafe {
                std::ptr::write(cell as *mut u32, min(last_best, last_cell) + row_item);
            }
            last_cell = *cell;
        }

        last_cell = u32::MAX - row[COL - 1];
        for (cell, row_item) in result_buffer.iter_mut().zip(row.iter().copied()).rev() {
            *cell = min(*cell, last_cell + row_item);
            last_cell = *cell;
        }

        println!("{:?}", result_buffer);
        result_buffer
    });

    last_row_risks[COL - 1] - grid[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chiton_path_risk_eg() {
        let cave_map = [
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let risk = path_risk(&cave_map);
        assert_eq!(risk, 40);
    }
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();

    let cave_map = read_input::<_, 100, 100>(stdin.lock()).unwrap();
    let risk = path_risk(&cave_map);
    println!("minimum path risk: {:?}", risk);
}
