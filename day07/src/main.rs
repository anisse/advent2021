fn main() {
    let heights = parse(include_str!("../input.txt"));
    //part 1
    let fuel = adjust_crabs(&heights);
    println!("Crab adjustment fuel: {}", fuel);
    //part 2
    let fuel = adjust_crabs2(&heights);
    println!("Crab adjustment fuel non constant: {}", fuel);
}
fn parse(input: &str) -> Vec<i16> {
    input
        .trim_end()
        .split(',')
        .map(|x| x.parse().expect("not int {}"))
        .collect()
}
fn adjust_crabs(heights: &[i16]) -> usize {
    let mut min = usize::MAX;
    let range = 0..*heights.iter().max().expect("max") as usize;
    for i in range {
        let mut total = 0;
        for h in heights.iter() {
            total += (*h as i32 - i as i32).abs();
        }
        if (total as usize) < min {
            min = total as usize;
        }
    }
    min
}

fn adjust_crabs2(heights: &[i16]) -> usize {
    let mut min = usize::MAX;
    let range = 0..*heights.iter().max().expect("max") as usize;
    for i in range {
        let mut total = 0;
        for h in heights.iter() {
            total += cost(*h, i);
        }
        if (total as usize) < min {
            min = total as usize;
        }
    }
    min
}

fn cost(x: i16, d: usize) -> usize {
    let diff = (x as i32 - d as i32).abs() as usize;
    diff * (diff + 1) / 2
}

#[test]
fn test() {
    let heights = parse(include_str!("../sample.txt"));
    //part 1
    let fuel = adjust_crabs(&heights);
    assert_eq!(fuel, 37);
    //part 2
    let fuel = adjust_crabs2(&heights);
    assert_eq!(fuel, 168);
}
