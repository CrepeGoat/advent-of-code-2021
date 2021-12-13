use aoc_lib::grid::{read_input, Grid};

fn advance_epoch<T, const ROW: usize, const COL: usize>(
    mut mat: Grid<T, ROW, COL>,
) -> Grid<T, ROW, COL> {
    unimplemented!()
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

    let dumbo_grid = read_input::<_, 10, 10>(stdin.lock()).unwrap();
    println!("dumbo grid: {:?}", dumbo_grid);
}
