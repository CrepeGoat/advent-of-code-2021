use aoc_lib::utils::parsing_input;

#[derive(Debug, PartialEq, Eq)]
enum Bracket {
    Parenthesis,
    Square,
    Curly,
    Angle,
}

struct ErrorTracker {
    stack: Vec<Bracket>,
}

impl ErrorTracker {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn advance(&mut self, c: char) -> Result<Option<Bracket>, &'static str> {
        use Bracket::*;
        let (bracket, is_closing) = match c {
            '(' => Ok((Parenthesis, false)),
            '[' => Ok((Square, false)),
            '{' => Ok((Curly, false)),
            '<' => Ok((Angle, false)),

            ')' => Ok((Parenthesis, true)),
            ']' => Ok((Square, true)),
            '}' => Ok((Curly, true)),
            '>' => Ok((Angle, true)),

            _ => Err("invalid character"),
        }?;

        Ok(if is_closing {
            if let Some(open_bracket) = self.stack.pop() {
                (bracket != open_bracket).then(|| bracket)
            } else {
                Some(bracket)
            }
        } else {
            self.stack.push(bracket);
            None
        })
    }
}

fn error_score(s: &str) -> Result<u64, &'static str> {
    let mut error_tracker = ErrorTracker::new();

    let mut result = 0;
    for r in s.chars().map(|c| error_tracker.advance(c)) {
        use Bracket::*;
        result += match r? {
            Some(Parenthesis) => 3,
            Some(Square) => 57,
            Some(Curly) => 1197,
            Some(Angle) => 25137,
            None => 0,
        };
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_diffs() {
        let sequence = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        let result = sequence
            .into_iter()
            .map(error_score)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .sum::<u64>();

        assert_eq!(result, 26397);
    }
}

fn main() {
    println!("Enter input sequence: ");
    let stdin = std::io::stdin();
    let parsed_inputs = parsing_input::<_, String>(stdin.lock());

    let result = parsed_inputs
        .into_iter()
        .map(|s| error_score(&s))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .sum::<u64>();
    println!("diffs count: {:?}", result);
}
