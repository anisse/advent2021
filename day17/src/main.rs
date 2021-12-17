struct Area {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}
fn main() {
    let area = parse(include_str!("../input.txt"));
    //part 1
    let highest = launch_highest(&area);
    println!("Highest possible: {}", highest);
    //part 2
    //let highest = launch_highest2(&area);
    //println!("Summary2: {}", highest);
}
fn parse(input: &str) -> Area {
    let xy: Vec<i32> = input
        .trim()
        .split(": ")
        .nth(1)
        .expect("riqht")
        .split(',')
        .map(|e| e.split('=').nth(1).expect("equal"))
        .map(|e| e.split("..").map(|s| s.parse::<i32>().expect("not int")))
        .flatten()
        .collect();
    //let x = xy.next().expect("x").split('=').nth(1).expect("value
    assert_eq!(xy.len(), 4);
    Area {
        xmin: *xy.get(0).unwrap(),
        xmax: *xy.get(1).unwrap(),
        ymin: *xy.get(2).unwrap(),
        ymax: *xy.get(3).unwrap(),
    }
}
fn launch_highest(area: &Area) -> i32 {
    let mut count = 0;
    let mut y = -area.ymin - 1;
    while y > 0 {
        count += y;
        y -= 1;
    }
    count
}

#[test]
fn test() {
    let area = parse(include_str!("../sample.txt"));
    //part 1
    let highest = launch_highest(&area);
    assert_eq!(highest, 45);
    //part 2
    // let highest = launch_highest2(&area);
    // assert_eq!(highest, 42);
}
