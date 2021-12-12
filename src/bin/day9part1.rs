use std::io::BufRead;

struct ArrayWrapper<T, const LEN: usize>([T; LEN]);

impl<T, const LEN: usize> std::iter::FromIterator<T> for ArrayWrapper<T, LEN> {
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let mut result: [T; LEN] = unsafe { std::mem::uninitialized() };

        let mut write_len = 0;
        for (i, item) in iterable.into_iter().enumerate() {
            unsafe {
                std::ptr::write(&mut result[i], item);
            }
            write_len += 1;
        }
        if write_len != LEN {
            panic!("wrong number of entries found; expected {:?}", LEN);
        }

        Self(result)
    }
}

impl<T, const LEN: usize> From<ArrayWrapper<T, LEN>> for [T; LEN] {
    fn from(wrapper: ArrayWrapper<T, LEN>) -> Self {
        wrapper.0
    }
}

type Matrix<T, const ROW: usize, const COL: usize> = [[T; COL]; ROW];

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
                _ => (),
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
        .zip(0..COL)
        .filter_map(move |(i, j)| low_points[i][j].then(|| 1 + cave_map[i][j]))
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
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();

    let cave_map = read_input::<_, 100, 100>(stdin.lock()).unwrap();
    let low_points = find_low_points(&cave_map);
    let result: u32 = risk_levels(&cave_map, &low_points).sum();
    println!("counted 1478: {:?}", result);
}
