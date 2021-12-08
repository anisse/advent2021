struct Entry {
    input: [String; 10],
    output: [String; 4],
}

fn main() {
    let digits = parse(include_str!("../input.txt"));
    //part 1
    let res = count_easy(&digits);
    println!("Summary: {}", res);
    //part 2
    //let res = count_easy2(&diag);
    //println!("Summary2: {}", res);
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

#[test]
fn test() {
    let digits = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_easy(&digits);
    assert_eq!(res, 26);
    //part 2
    // let res = count_easy2(&digits);
    // assert_eq!(res, 42);
}
