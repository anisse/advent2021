fn main() {
    let heightmap = parse(include_str!("../input.txt"));
    //part 1
    let risk = lowpointsrisk(&heightmap);
    println!("Risk sum: {}", risk);
    //part 2
    //let risk = lowpointsrisk2(&heightmap);
    //println!("Risk sum2: {}", risk);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().expect("not int"))
                .collect()
        })
        .collect()
}
fn lowpointsrisk(heightmap: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for (y, row) in heightmap.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if is_low_point(heightmap, y, x) {
                count += *h as usize + 1;
            }
        }
    }
    count
}

fn is_low_point(heightmap: &[Vec<u8>], y: usize, x: usize) -> bool {
    let val = heightmap[y][x];
    if y > 0 && heightmap[y - 1][x] <= val {
        return false;
    }
    if x > 0 && heightmap[y][x - 1] <= val {
        return false;
    }
    if y < heightmap.len() - 1 && heightmap[y + 1][x] <= val {
        return false;
    }
    if x < heightmap[y].len() - 1 && heightmap[y][x + 1] <= val {
        return false;
    }
    true
}

#[test]
fn test() {
    let heightmap = parse(include_str!("../sample.txt"));
    //part 1
    let risk = lowpointsrisk(&heightmap);
    assert_eq!(risk, 15);
    //part 2
    // let risk = lowpointsrisk2(&heightmap);
    // assert_eq!(risk, 42);
}
