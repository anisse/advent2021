fn main() {
    let things = parse(include_str!("../input.txt"));
    //part 1
    let res = operation(&things);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&diag);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn operation(things: &[u8]) -> usize {
    todo!()
}

#[test]
fn test() {
    let things = parse(include_str!("../sample.txt"));
    //part 1
    let res = operation(&things);
    assert_eq!(res, 42);
    //part 2
    // let res = operation2(&things);
    // assert_eq!(res, 42);
}
