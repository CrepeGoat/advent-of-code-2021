use aoc_lib::utils::{ArrayWrapper, Matrix};

use std::io::BufRead;

fn read_input<R: BufRead, const ROW: usize, const COL: usize>(
    reader: R,
) -> Result<Matrix<u32, ROW, COL>, &'static str> {
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

fn find_low_points<const ROW: usize, const COL: usize>(
    cave_map: &Matrix<u32, ROW, COL>,
) -> Matrix<bool, ROW, COL> {
    use std::cmp::Ordering::*;

    let mut result = [[true; COL]; ROW];

    for (res_row, cave_row) in (&mut result[..]).iter_mut().zip(&cave_map[..]) {
        for j in 0..(COL - 1) {
            match cave_row[j].cmp(&cave_row[j + 1]) {
                Less => res_row[j + 1] = false,
                Greater => res_row[j] = false,
                _ => {
                    res_row[j] = false;
                    res_row[j + 1] = false;
                }
            }
        }
    }

    for j in 0..COL {
        for i in 0..(ROW - 1) {
            match cave_map[i][j].cmp(&cave_map[i + 1][j]) {
                Less => result[i + 1][j] = false,
                Greater => result[i][j] = false,
                _ => (),
            }
        }
    }

    result
}

fn risk_levels<'a, const ROW: usize, const COL: usize>(
    cave_map: &'a Matrix<u32, ROW, COL>,
    low_points: &'a Matrix<bool, ROW, COL>,
) -> impl 'a + Iterator<Item = u32> {
    (0..ROW)
        .flat_map(|i| (0..COL).map(move |j| (i, j)))
        .filter_map(move |(i, j)| low_points[i][j].then(|| 1 + cave_map[i][j]))
}

fn adj_coords<const ROW: usize, const COL: usize>(
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

fn iter_basins<const ROW: usize, const COL: usize>(
    cave_map: &Matrix<u32, ROW, COL>,
    mut low_points: Matrix<bool, ROW, COL>,
) -> impl '_ + Iterator<Item = usize> {
    let iter_low_points = (0..ROW)
        .flat_map(|i| (0..COL).map(move |j| (i, j)))
        .filter(|(i, j)| low_points[*i][*j])
        .collect::<Vec<_>>();

    let mut point_buffer = Vec::new();
    iter_low_points.into_iter().map(move |low_point| {
        let mut count = 0;
        point_buffer.push(low_point);

        while let Some(coord) = point_buffer.pop() {
            count += 1;

            for (i, j) in adj_coords::<ROW, COL>(coord) {
                if low_points[i][j]
                    || cave_map[i][j] >= 9
                    || cave_map[i][j] < cave_map[coord.0][coord.1]
                {
                    continue;
                }
                point_buffer.push((i, j));
                low_points[i][j] = true;
            }
        }

        count
    })
}

fn n_min<T: core::cmp::Ord, I: Iterator<Item = T>>(n: usize, mut iter: I) -> Vec<T> {
    let mut buffer = std::collections::BinaryHeap::new();

    for item in iter.by_ref().take(n) {
        buffer.push(item);
    }
    for item in iter {
        buffer.push(item);
        buffer.pop();
    }

    buffer.into_sorted_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_points_example() {
        let cave_map = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let low_points = find_low_points(&cave_map);
        assert_eq!(
            low_points,
            [
                [false, true, false, false, false, false, false, false, false, true],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, true, false, false, false],
            ]
        );
    }

    #[test]
    fn test_risk_levels_example() {
        let cave_map = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let low_points = find_low_points(&cave_map);
        assert_eq!(
            risk_levels(&cave_map, &low_points).collect::<Vec<_>>(),
            vec![2, 1, 6, 6]
        );
    }

    #[test]
    fn test_iter_basins_example() {
        let cave_map = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let low_points = find_low_points(&cave_map);
        assert_eq!(
            iter_basins(&cave_map, low_points).collect::<Vec<_>>(),
            vec![3, 9, 14, 9],
        );
    }
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();

    let cave_map = read_input::<_, 100, 100>(stdin.lock()).unwrap();
    let low_points = find_low_points(&cave_map);
    let basins: Vec<_> = n_min(3, iter_basins(&cave_map, low_points).map(std::cmp::Reverse))
        .into_iter()
        .map(|r| r.0)
        .collect();
    println!("basins: {:?}", basins);
    println!("basin product: {:?}", basins.into_iter().product::<usize>());
}
