use integer_sqrt::IntegerSquareRoot;

use aoc_lib::_2d_int::*;

use core::ops::Range;
use std::convert::TryInto;

fn arc_pos(vel: i64, t: usize) -> i64 {
    // x(t) = t(2v + 1 - t) / 2
    let t: i64 = t.try_into().expect("time is too large");

    (t * (2 * vel + 1 - t)) / 2
}

fn arc_1st_xing(vel: i64, target: i64) -> (Option<i64>, Option<i64>) {
    /*
        Calculate the rising index at which the arc will cross a given target value.
    */
    // x(t) = t(2v + 1 - t) / 2
    // => t = (2v + 1 ±√((2v+1)² - 8x)) / 2
    let _2vp1 = 2 * vel + 1;
    let r_pos = _2vp1 * _2vp1;
    let r_neg = 8 * target;

    if r_pos < r_neg {
        return (None, None);
    }

    let root = (r_pos - r_neg).integer_sqrt();

    let fall_idx = (_2vp1 + root >= 0).then(|| (_2vp1 + root + 1) / 2);
    let rise_idx = fall_idx.and_then(|_| (_2vp1 >= root).then(|| (_2vp1 - root) / 2));
    (rise_idx, fall_idx)
}

fn target_overlap_x(vel: i64, pos: i64, target: Range<i64>) -> Range<usize> {
    let peak_dist = (vel * (i64::abs(vel) + 1)) / 2;
    let peak_idx: usize = i64::abs(vel) as usize;

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
