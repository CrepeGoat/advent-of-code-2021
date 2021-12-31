use core::convert::TryFrom;
use integer_sqrt::IntegerSquareRoot;

use aoc_lib::_2d_int::*;

use core::ops::RangeInclusive;
use std::convert::TryInto;

fn arc_pos(vel: i64, t: usize) -> i64 {
    // x(t) = t(2v + 1 - t) / 2
    let t: i64 = t.try_into().expect("time is too large");

    (t * (2 * vel + 1 - t)) / 2
}

fn arc_xings(vel: i64, target: i64) -> (Option<usize>, Option<usize>) {
    /*
        Calculate the rising index at which the arc will cross a given target value.

        As the result is represented in integers, the lower bound is guaranteed to be
        equal or less than the true value, and the upper bound is guaranteed to be
        equal or greater than the true value.
    */
    // x(t) = t(2v + 1 - t) / 2
    // => t = (2v + 1 ±√((2v+1)² - 8x)) / 2
    let _2vp1 = 2 * vel + 1;

    if let Some(root) = (_2vp1 * _2vp1 - 8 * target).integer_sqrt_checked() {
        let fall_idx = (_2vp1 + root >= 0).then(|| (-(_2vp1 + root)).div_euclid(-2));
        let rise_idx = fall_idx.and_then(|_| (_2vp1 >= root).then(|| (_2vp1 - root).div_euclid(2)));

        (
            rise_idx.map(|t| usize::try_from(t).unwrap()),
            fall_idx.map(|t| usize::try_from(t).unwrap()),
        )
    } else {
        (None, None)
    }
}

fn xing_vels(
    target: (RangeInclusive<i64>, RangeInclusive<i64>),
) -> impl Iterator<Item = (i64, i64)> {
    let vel_y0 = (*target.1.start() < 0)
        .then(|| *target.1.start())
        .unwrap_or_default();
    let vel_y1 = (*target.1.end() > 0)
        .then(|| *target.1.end())
        .unwrap_or_default();

    (vel_y0..=vel_y1).rev().flat_map(|vel_y| {})
}

fn target_overlap_x(vel: i64, pos: i64, target: RangeInclusive<i64>) -> RangeInclusive<i64> {
    unimplemented!()
}

fn flight_path(mut vel: Vector<i64>, mut pos: Point<i64>) -> impl Iterator<Item = Point<i64>> {
    core::iter::once(pos).chain(core::iter::from_fn(move || {
        pos = pos + vel;

        use core::cmp::Ordering::*;
        vel.x = vel.x
            + match vel.x.cmp(&0) {
                Greater => -1,
                Less => 1,
                Equal => 0,
            };
        vel.y -= 1;

        Some(pos)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eg() {
        let x_range = 20..=30;
        let y_range = -10..=-5;

        let calc_result = xing_vels((x_range, y_range)).collect::<Vec<_>>();
        let expt_result = vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];

        assert_eq!(calc_result.len(), 112);
        assert_eq!(
            calc_result
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
            expt_result
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
        )
    }

    #[test]
    fn test_input() {
        let x_range = 135..=155;
        let y_range = -102..=-78;
    }
}
