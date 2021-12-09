use crate::Segment::*;
use std::collections::{HashMap, HashSet};

type DigitUnknown = HashSet<char>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Segment {
    A = (1 << 0),
    B = (1 << 1),
    C = (1 << 2),
    D = (1 << 3),
    E = (1 << 4),
    F = (1 << 5),
    G = (1 << 6),
}

struct Entry {
    input: [String; 10],
    output: [String; 4],
}

fn main() {
    let digits = parse(include_str!("../input.txt"));
    //part 1
    let res = count_easy(&digits);
    println!("Count easy numbers: {}", res);
    //part 2
    let res = count_outputs(&digits);
    println!("Output sum: {}", res);
}
fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|x| {
            let mut e = x.split(" | ");
            let input = e.next().expect("missing input");
            let output = e.next().expect("missing output");
            Entry {
                input: input
                    .split_ascii_whitespace()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .try_into()
                    .expect("bad slice input ?"),
                output: output
                    .split_ascii_whitespace()
                    .map(|o| o.to_string())
                    .collect::<Vec<String>>()
                    .try_into()
                    .expect("bad slice output ?"),
            }
        })
        .collect()
}
fn count_easy(digits: &[Entry]) -> usize {
    let mut count = 0;
    for e in digits.iter() {
        for d in e.output.iter() {
            match d.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }
    count
}
fn count_outputs(digits: &[Entry]) -> usize {
    let mut total = 0;
    for e in digits.iter() {
        let segments = segment_map(&e.input);
        let mut val = 0;
        for (i, d) in e.output.iter().rev().enumerate() {
            val += digit(d, &segments) as usize * 10_usize.pow(i as u32);
        }
        total += val;
    }

    total
}

fn segment_map(input: &[String; 10]) -> HashMap<char, Segment> {
    /* Hand-rolled, constant-time paper-found "a little deduction"
     *
     * Not even sure it is faster than brute forcing the solution...
     */
    let input: Vec<DigitUnknown> = input.iter().map(|d| du(d.clone())).collect();
    let mut segments: HashMap<char, Segment> = HashMap::new();
    let a1 = input.iter().find(|d| d.len() == 2).expect("no 1");
    let a4 = input.iter().find(|d| d.len() == 4).expect("no 4");
    let a7 = input.iter().find(|d| d.len() == 3).expect("no 7");
    let a8 = input.iter().find(|d| d.len() == 7).expect("no 8");
    let a235 = input
        .iter()
        .filter(|d| d.len() == 5)
        .cloned()
        .reduce(|sum, i| sum.intersection(&i).cloned().collect::<HashSet<char>>())
        .expect("no 2,3,5 intersection");
    let a069 = input
        .iter()
        .filter(|d| d.len() == 6)
        .cloned()
        .reduce(|sum, i| sum.intersection(&i).cloned().collect::<HashSet<char>>())
        .expect("no 0,6,9 intersection");

    let char_a = *a7.difference(a1).next().expect("no a");
    segments.insert(char_a, A);
    let char_e = *a8
        .difference(a4)
        .cloned()
        .collect::<DigitUnknown>()
        .difference(&a235)
        .next()
        .expect("no e");
    segments.insert(char_e, E);
    let char_g = *a8
        .difference(&a4.union(a7).cloned().collect::<DigitUnknown>())
        .cloned()
        .collect::<DigitUnknown>()
        .difference(&DigitUnknown::from([char_e]))
        .next()
        .expect("no g");
    segments.insert(char_g, G);
    let char_d = *a235
        .difference(&DigitUnknown::from([char_a, char_g]))
        .next()
        .expect("no d");
    segments.insert(char_d, D);
    let char_b = *a4
        .difference(a1)
        .cloned()
        .collect::<DigitUnknown>()
        .difference(&DigitUnknown::from([char_d]))
        .next()
        .expect("no b");
    segments.insert(char_b, B);
    let char_f = *a069
        .difference(&DigitUnknown::from([char_a, char_b, char_g]))
        .next()
        .expect("no f");
    segments.insert(char_f, F);
    let char_c = *a1
        .difference(&DigitUnknown::from([char_f]))
        .next()
        .expect("no c");
    segments.insert(char_c, C);
    segments
}

fn du(d: String) -> DigitUnknown {
    let mut digit = DigitUnknown::new();
    for c in d.chars() {
        digit.insert(c);
    }
    digit
}

fn digit(d: &str, s: &HashMap<char, Segment>) -> u8 {
    let numbers: HashMap<usize, u8> = HashMap::from([
        (bitval(&[A, B, C, E, F, G]), 0),
        (bitval(&[C, F]), 1),
        (bitval(&[A, C, D, E, G]), 2),
        (bitval(&[A, C, D, F, G]), 3),
        (bitval(&[B, C, D, F]), 4),
        (bitval(&[A, B, D, F, G]), 5),
        (bitval(&[A, B, D, E, F, G]), 6),
        (bitval(&[A, C, F]), 7),
        (bitval(&[A, B, C, D, E, F, G]), 8),
        (bitval(&[A, B, C, D, F, G]), 9),
    ]);
    let segval = d
        .chars()
        .map(|x| *s.get(&x).expect("not found in map") as usize)
        .sum();
    *numbers.get(&segval).expect("no segment to num match")
}

fn bitval(a: &[Segment]) -> usize {
    a.iter().map(|x| *x as usize).sum()
}

#[test]
fn test() {
    let digits = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_easy(&digits);
    assert_eq!(res, 26);
    //part 2
    let res = count_outputs(&digits);
    assert_eq!(res, 61229);
}
