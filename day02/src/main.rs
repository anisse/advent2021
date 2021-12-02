enum Ins {
    Forward(u8),
    Down(u8),
    Up(u8),
}

fn main() {
    let course = parse(include_str!("../input.txt"));
    //part 1
    let pos = compute_position(&course);
    println!("Position: {}", pos);
    //part 2
    let pos = compute_with_aim(&course);
    println!("Position with aim: {}", pos);
}

fn parse(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|x| {
            let mut words = x.split_ascii_whitespace();
            let cmd = words.next().expect("missing command");
            let arg = words
                .next()
                .expect("missing arg")
                .parse::<u8>()
                .expect("arg not integer");
            match cmd {
                "forward" => Ins::Forward(arg),
                "down" => Ins::Down(arg),
                "up" => Ins::Up(arg),
                _ => panic!("unknown cmd {}", cmd),
            }
        })
        .collect()
}
fn compute_position(course: &[Ins]) -> usize {
    let mut x = 0_usize;
    let mut y = 0_usize;
    for i in course.iter() {
        match i {
            Ins::Forward(a) => x += *a as usize,
            Ins::Down(a) => y += *a as usize,
            Ins::Up(a) => y -= *a as usize,
        }
    }
    x * y
}

fn compute_with_aim(course: &[Ins]) -> usize {
    let mut x = 0_usize;
    let mut y = 0_usize;
    let mut aim = 0_usize;
    for i in course.iter() {
        match i {
            Ins::Down(a) => aim += *a as usize,
            Ins::Up(a) => aim -= *a as usize,
            Ins::Forward(a) => {
                x += *a as usize;
                y += aim * *a as usize;
            }
        }
    }
    x * y
}

#[test]
fn test() {
    let course = parse(include_str!("../sample.txt"));
    let pos = compute_position(&course);
    assert_eq!(pos, 150);
    let pos = compute_with_aim(&course);
    assert_eq!(pos, 900);
}
