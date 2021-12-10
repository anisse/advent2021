fn main() {
    let subsystems = parse(include_str!("../input.txt"));
    //part 1
    let score = syntax_errors(&subsystems);
    println!("Error score: {}", score);
    //part 2
    let score = incompletes(&subsystems);
    println!("Incomplete score: {}", score);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|x| x.bytes().collect()).collect()
}

fn syntax_errors(subsystems: &[Vec<u8>]) -> usize {
    let mut score = 0;
    for s in subsystems.iter() {
        let mut stack: Vec<u8> = Vec::new();

        //let st = s.iter().map(|x| *x as char).collect::<String>();
        //for (i, c) in s.iter().enumerate() {
        for c in s.iter() {
            if let Some(m) = close_match(*c) {
                let mut illegal = false;
                if let Some(prev) = stack.pop() {
                    if prev != m {
                        illegal = true;
                        /*
                        println!();
                        println!("{}", st);
                        println!(
                            "{}{}",
                            stack.iter().map(|x| *x as char).collect::<String>(),
                            prev as char
                        );
                        println!("{}", st.get(0..=i).unwrap());
                        println!("Got {} to close {}", *c as char, prev as char);
                        */
                    } /*else {
                          println!(
                              "{}{}",
                              (0..stack.len()).map(|_| ' ').collect::<String>(),
                              *c as char
                          );
                      }
                        */
                } else {
                    illegal = true;
                    //println!("Closing with no match");
                }
                if illegal {
                    score += close_score(*c);
                    //println!("illegal {} ({})", *c as char, close_score(*c));
                    break;
                }
            } else {
                /*
                println!(
                    "{}{}",
                    (0..stack.len()).map(|_| ' ').collect::<String>(),
                    *c as char
                );
                */
                stack.push(*c);
            }
        }
    }
    score
}

fn close_match(c: u8) -> Option<u8> {
    match c {
        b'}' => Some(b'{'),
        b']' => Some(b'['),
        b')' => Some(b'('),
        b'>' => Some(b'<'),
        _ => None,
    }
}
fn close_score(c: u8) -> usize {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

fn incompletes(subsystems: &[Vec<u8>]) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for s in subsystems.iter() {
        let mut stack: Vec<u8> = Vec::new();
        let mut illegal = false;

        for c in s.iter() {
            if let Some(m) = close_match(*c) {
                if let Some(prev) = stack.pop() {
                    if prev != m {
                        illegal = true;
                    }
                } else {
                    illegal = true;
                }
                if illegal {
                    break;
                }
            } else {
                stack.push(*c);
            }
        }
        if !illegal {
            scores.push(incomplete_score(&stack))
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn incomplete_score(stack: &[u8]) -> usize {
    let mut score = 0;
    for c in stack.iter().rev() {
        let s = match c {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        };
        score = score * 5 + s;
    }
    score
}

#[test]
fn test() {
    let subsystems = parse(include_str!("../sample.txt"));
    //part 1
    let score = syntax_errors(&subsystems);
    assert_eq!(score, 26397);
    //part 2
    let score = incompletes(&subsystems);
    assert_eq!(score, 288957);
}
