use crate::Segment::*;
use std::collections::HashMap;

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

fn segment_map(input: &[String; 10]) -> HashMap<u8, Segment> {
    /* Hand-rolled, constant-time paper-found "a little deduction"
     *
     * Not even sure it is faster than brute forcing the solution...
     */
    let input: Vec<u8> = input.iter().map(|d| d2bits(d)).collect();
    let mut segments: HashMap<u8, Segment> = HashMap::new();
    let a1 = *input.iter().find(|d| d.count_ones() == 2).expect("no 1");
    let a4 = *input.iter().find(|d| d.count_ones() == 4).expect("no 4");
    let a7 = *input.iter().find(|d| d.count_ones() == 3).expect("no 7");
    let a8 = *input.iter().find(|d| d.count_ones() == 7).expect("no 8");
    let a235 = input
        .clone()
        .into_iter()
        .filter(|d| d.count_ones() == 5)
        .reduce(|sum, i| sum & i)
        .expect("no 2,3,5 intersection");
    let a069 = input
        .into_iter()
        .filter(|d| d.count_ones() == 6)
        .reduce(|sum, i| sum & i)
        .expect("no 0,6,9 intersection");

    fn diff(a: u8, b: u8) -> u8 {
        (a ^ b) & a
    }
    let char_a = diff(a7, a1);
    check_single(char_a);
    segments.insert(char_a, A);
    let char_e = diff(diff(a8, a4), a235);
    check_single(char_e);
    segments.insert(char_e, E);
    let char_g = diff(diff(a8, a4 | a7), char_e);
    check_single(char_g);
    segments.insert(char_g, G);

    let char_d = diff(a235, char_a | char_g);
    check_single(char_d);
    segments.insert(char_d, D);
    let char_b = diff(diff(a4, a1), char_d);
    check_single(char_b);
    segments.insert(char_b, B);
    let char_f = diff(a069, char_a | char_b | char_g);
    check_single(char_f);
    segments.insert(char_f, F);
    let char_c = diff(a1, char_f);
    check_single(char_c);
    segments.insert(char_c, C);
    segments
}

fn d2bits(d: &str) -> u8 {
    d.bytes().map(c2bit).sum()
}

fn c2bit(c: u8) -> u8 {
    1 << (c - b'a')
}

fn check_single(d: u8) {
    if d.count_ones() != 1 {
        panic!("{} should only have one bit set", d)
    }
}

fn digit(d: &str, s: &HashMap<u8, Segment>) -> u8 {
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
        .bytes()
        .map(|x| *s.get(&c2bit(x)).expect("not found in map") as usize)
        .sum();
    *numbers.get(&segval).expect("no segment to num match")
}

const fn bitval(a: &[Segment]) -> usize {
    let mut sum = 0;
    let mut i = 0;
    loop {
        sum += a[i] as usize;
        i += 1;
        if i == a.len() {
            break;
        }
    }
    sum
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
