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
    let total = velocities_on_target(&area);
    println!("Number of possible velocities: {}", total);
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
fn velocities_on_target(area: &Area) -> usize {
    let ymax = -area.ymin - 1;
    let ymin = area.ymin;
    let xmin = xmin(area.xmin);
    let xmax = area.xmax;
    assert!(xmin < xmax);
    assert!(ymin < ymax);
    let mut count = 0;
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            if sim(area, x, y) {
                //println!("v=({}, {})", x, y);
                count += 1;
            }
        }
    }
    count
}

fn xmin(x: i32) -> i32 {
    let mut min = (x, x * 2);
    for i in (0..(x / 2 + 1)).rev() {
        let c = count_down(i);
        if i < min.0 && c >= x {
            min = (i, c);
        }
    }
    min.0
}
fn count_down(mut x: i32) -> i32 {
    let mut count = 0;
    while x > 0 {
        count += x;
        x -= 1;
    }
    count
}

fn sim(a: &Area, mut vx: i32, mut vy: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <= a.xmax && y >= a.ymin {
        if x >= a.xmin && x <= a.xmax && y >= a.ymin && y <= a.ymax {
            return true;
        }
        x += vx;
        y += vy;
        vx = match vx.cmp(&0) {
            std::cmp::Ordering::Less => vx + 1,
            std::cmp::Ordering::Equal => vx,
            std::cmp::Ordering::Greater => vx - 1,
        };
        vy -= 1;
    }
    false
}

#[test]
fn test() {
    assert_eq!(xmin(20), 6);
    let area = parse(include_str!("../sample.txt"));
    //part 1
    let highest = launch_highest(&area);
    assert_eq!(highest, 45);
    //part 2
    let total = velocities_on_target(&area);
    assert_eq!(total, 112);
}
