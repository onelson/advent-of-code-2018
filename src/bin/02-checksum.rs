use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Given a str and the unique characters in it, check to see if any of them
/// match exactly the number of times specified.
fn has_exact_count(count: usize, s: &str, chars: &HashSet<char>) -> usize {
    if chars
        .iter()
        .any(|c1| s.chars().filter(|c2| c1 == c2).count() == count)
    {
        1
    } else {
        0
    }
}

fn main() {
    let fp = env::args().skip(1).next().expect("file path");
    let f = File::open(fp).unwrap();
    let reader = BufReader::new(f);

    let ret: (usize, usize) = reader
        .lines()
        .map(|x| {
            let input = x.unwrap();
            let chars: HashSet<_> = input.chars().collect();
            (
                has_exact_count(2, &input, &chars),
                has_exact_count(3, &input, &chars),
            )
        }).fold((0, 0), |acc, t| (acc.0 + t.0, acc.1 + t.1));

    println!("{} * {} = {}", ret.0, ret.1, ret.0 * ret.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "aabbcccc";

    #[test]
    fn test_match_pairs() {
        let chars: HashSet<_> = INPUT.chars().collect();
        // there are two pairs, but we only count the first one.
        assert_eq!(1, has_exact_count(2, INPUT, &chars));
    }

    #[test]
    fn test_match_triples() {
        let chars: HashSet<_> = INPUT.chars().collect();
        assert_eq!(0, has_exact_count(3, INPUT, &chars));
    }

    #[test]
    fn test_match_quads() {
        let chars: HashSet<_> = INPUT.chars().collect();
        assert_eq!(1, has_exact_count(4, INPUT, &chars));
    }
}
