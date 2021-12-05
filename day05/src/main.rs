#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}
type Line = [Point; 2];

fn main() {
    let lines = parse(include_str!("../input.txt"));
    //part 1
    let points = axes_lines_overlap(&lines);
    println!("Overlapping points from axes-parallel lines: {}", points);
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let coord: Vec<u16> = p
                        .split(',')
                        .map(|x| x.parse::<u16>().expect("not int"))
                        .collect();
                    assert_eq!(coord.len(), 2);
                    Point {
                        x: coord[0],
                        y: coord[1],
                    }
                })
                .collect::<Vec<Point>>()
                .try_into()
                .expect("Vec to array fail")
        })
        .collect()
}

fn axes_lines_overlap(lines: &[Line]) -> usize {
    // lazy double iteration for max
    let max_x = lines
        .iter()
        .flat_map(|l| l.iter())
        .map(|p| p.x)
        .max()
        .expect("no x ?") as usize;
    let max_y = lines
        .iter()
        .flat_map(|l| l.iter())
        .map(|p| p.y)
        .max()
        .expect("no y ?") as usize;
    let mut floor = vec![vec![0; max_x + 1]; max_y + 1];

    let mut count = 0;

    for l in lines.iter().filter(|l| axes_line(l)) {
        count += draw_line_axes(&mut floor, l);
    }

    count
}

fn axes_line(l: &Line) -> bool {
    l[0].x == l[1].x || l[0].y == l[1].y
}

fn draw_line_axes(floor: &mut [Vec<u8>], l: &Line) -> usize {
    let mut overlaps = 0;
    for x in line_range(l[0].x, l[1].x) {
        for y in line_range(l[0].y, l[1].y) {
            floor[y][x] += 1;
            if floor[y][x] == 2 {
                overlaps += 1
            }
        }
    }
    overlaps
}

fn line_range(end1: u16, end2: u16) -> std::ops::RangeInclusive<usize> {
    end1.min(end2) as usize..=end2.max(end1) as usize
}

#[test]
fn test() {
    let lines = parse(include_str!("../sample.txt"));
    //part 1
    let points = axes_lines_overlap(&lines);
    assert_eq!(points, 5);
}
