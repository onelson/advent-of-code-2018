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

fn similar(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
}

fn main() {
    let fp = env::args().skip(1).next().expect("file path");
    let f = File::open(fp).unwrap();
    let reader = BufReader::new(f);

    let ids: Vec<_> = reader.lines().map(|x| x.unwrap()).collect();

    let mut pair: (String, String) = ("".to_string(), "".to_string());
    'outer: for id1 in &ids {
        for id2 in &ids {
            if similar(id1, id2) {
                pair = (id1.to_string(), id2.to_string());
                break 'outer;
            }
        }
    }
    let ret: String = pair
        .0
        .chars()
        .zip(pair.1.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect();
    println!("{}", ret);
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

    #[test]
    fn test_similars() {
        assert!(similar("fghij", "fguij"));
        assert!(similar("fghij", "fghiy"));
        assert!(similar("pghij", "fghij"));
    }

    #[test]
    fn test_disimilars() {
        // two different
        assert!(!similar("fghij", "fguiy"));
        // identicals are not similars
        assert!(!similar("fghij", "fghij"));
    }
}
