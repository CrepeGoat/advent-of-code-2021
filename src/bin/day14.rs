use aoc_lib::utils::{n_min, ArrayWrapper};
use core::hash::Hash;
use core::str::FromStr;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

struct Insertion {
    first: char,
    last: char,
    insert: char,
}

impl FromStr for Insertion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err("wrong string length");
        }
        match s.chars().collect::<ArrayWrapper<char, 7>>() {
            ArrayWrapper([first, last, ' ', '-', '>', ' ', insert]) => Ok(Self {
                first,
                last,
                insert,
            }),
            _ => Err("wrong format"),
        }
    }
}

fn read_input<R: BufRead>(mut reader: R) -> Result<(String, Vec<Insertion>), &'static str> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).map_err(|_| "bad read")?;

    let strings = buffer.split("\n\n").collect::<ArrayWrapper<&str, 2>>().0;

    let initial_seq = strings[0].trim();
    let insertions: Vec<_> = strings[1]
        .trim()
        .split('\n')
        .map(Insertion::from_str)
        .collect::<Result<_, _>>()?;

    Ok((initial_seq.to_string(), insertions))
}

type PolymerInsertionMap = HashMap<(char, char), String>;

fn new_map<I: Iterator<Item = Insertion>>(insertions: I) -> PolymerInsertionMap {
    insertions
        .map(|insert| ((insert.first, insert.last), insert.insert.to_string()))
        .collect()
}

fn next_tier(insertion_map: &PolymerInsertionMap) -> PolymerInsertionMap {
    insertion_map
        .iter()
        .map(|(&k, v)| {
            let mut buffer = String::new();
            buffer.push(k.0);
            buffer.push_str(v);
            buffer.push(k.1);

            let mut result = apply_tier(&buffer, insertion_map);
            result.remove(0);
            result.pop();
            (k, result)
        })
        .collect()
}

fn apply_tier(pattern: &str, insertion_map: &PolymerInsertionMap) -> String {
    let chars = pattern.chars();

    pattern
        .chars()
        .map(|c| c.to_string())
        .interleave(
            chars
                .tuple_windows()
                .map(|k| insertion_map.get(&k).cloned().unwrap_or_default()),
        )
        .fold(String::new(), |mut result, s| {
            result.push_str(&s);
            result
        })
}

fn polymerized(
    mut template: String,
    mut insertion_map: PolymerInsertionMap,
    iterations: u32,
) -> String {
    let bit_len = 32 - iterations.leading_zeros();

    if iterations == 0 {
        return template;
    }

    for i in 0..(bit_len - 1) {
        if (iterations & (1 << i)) != 0 {
            template = apply_tier(&template, &insertion_map);
        }
        insertion_map = next_tier(&insertion_map);
    }
    template = apply_tier(&template, &insertion_map);

    template
}

fn counts<I>(iter: I) -> HashMap<<I as Iterator>::Item, usize>
where
    I: Iterator,
    <I as Iterator>::Item: Clone + Copy + Hash + std::cmp::Eq,
{
    iter.fold(HashMap::new(), |mut map, i| {
        map.insert(i, 1 + map.get(&i).cloned().unwrap_or_default());
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymerized_eg() {
        let template = "NNCB";
        let insertion_map = new_map(
            vec![
                Insertion {
                    first: 'C',
                    last: 'H',
                    insert: 'B',
                },
                Insertion {
                    first: 'H',
                    last: 'H',
                    insert: 'N',
                },
                Insertion {
                    first: 'C',
                    last: 'B',
                    insert: 'H',
                },
                Insertion {
                    first: 'N',
                    last: 'H',
                    insert: 'C',
                },
                Insertion {
                    first: 'H',
                    last: 'B',
                    insert: 'C',
                },
                Insertion {
                    first: 'H',
                    last: 'C',
                    insert: 'B',
                },
                Insertion {
                    first: 'H',
                    last: 'N',
                    insert: 'C',
                },
                Insertion {
                    first: 'N',
                    last: 'N',
                    insert: 'C',
                },
                Insertion {
                    first: 'B',
                    last: 'H',
                    insert: 'H',
                },
                Insertion {
                    first: 'N',
                    last: 'C',
                    insert: 'B',
                },
                Insertion {
                    first: 'N',
                    last: 'B',
                    insert: 'B',
                },
                Insertion {
                    first: 'B',
                    last: 'N',
                    insert: 'B',
                },
                Insertion {
                    first: 'B',
                    last: 'B',
                    insert: 'N',
                },
                Insertion {
                    first: 'B',
                    last: 'C',
                    insert: 'B',
                },
                Insertion {
                    first: 'C',
                    last: 'C',
                    insert: 'N',
                },
                Insertion {
                    first: 'C',
                    last: 'N',
                    insert: 'C',
                },
            ]
            .into_iter(),
        );

        let calc_result = polymerized(template.to_string(), insertion_map.clone(), 0);
        assert_eq!(calc_result, "NNCB".to_string(), "polymerize 0 times");

        let calc_result = polymerized(template.to_string(), insertion_map.clone(), 1);
        assert_eq!(calc_result, "NCNBCHB".to_string(), "polymerize 1 times");

        let calc_result = polymerized(template.to_string(), insertion_map.clone(), 2);
        assert_eq!(
            calc_result,
            "NBCCNBBBCBHCB".to_string(),
            "polymerize 2 times"
        );

        let calc_result = polymerized(template.to_string(), insertion_map.clone(), 3);
        assert_eq!(
            calc_result,
            "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string(),
            "polymerize 3 times"
        );

        let calc_result = polymerized(template.to_string(), insertion_map, 4);
        assert_eq!(
            calc_result,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string(),
            "polymerize 4 times"
        );
    }
}

fn main() {
    println!("Enter input sequence: ");
    let stdin = std::io::stdin();
    let (template, insertions) = read_input(stdin.lock()).unwrap();

    let polymer = polymerized(template, new_map(insertions.into_iter()), 10);
    //println!("polymer: {:?}", polymer);
    let element_counts = counts(polymer.chars());
    println!("polymer element counts: {:?}", element_counts);
    println!(
        "result = {:?}",
        element_counts.values().cloned().max().unwrap()
            - element_counts.values().cloned().min().unwrap()
    );
}
