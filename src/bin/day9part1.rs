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
type AnyError = Box<dyn std::error::Error>;

fn read_input<R: BufRead, T, const ROW: usize, const COL: usize>(
    reader: R,
) -> Result<Matrix<T, ROW, COL>, AnyError>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: 'static + std::error::Error,
{
    reader
        .lines()
        .map(|r| -> Result<_, AnyError> {
            r.map_err(From::from)?
                .chars()
                .map(|c| T::from_str(c.to_string().as_str()).map_err(From::from))
                .collect::<Result<ArrayWrapper<_, COL>, _>>()
                .map(From::from)
        })
        .collect::<Result<ArrayWrapper<_, ROW>, _>>()
        .map(From::from)
}
