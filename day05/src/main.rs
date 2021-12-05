#[derive(Debug, PartialEq, Clone, Copy)]
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
    let points = lines_overlap(&lines);
    println!(
        "Overlapping points from axes-parallel and diagonal lines: {}",
        points
    );
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
    let mut floor = floor_map_minimal(lines);
    let mut count = 0;

    for l in lines.iter().filter(|l| axes_line(l)) {
        count += draw_line_axes(&mut floor, l);
    }

    count
}

fn floor_map_minimal(lines: &[Line]) -> Vec<Vec<u8>> {
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
    vec![vec![0; max_x + 1]; max_y + 1]
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

fn lines_overlap(lines: &[Line]) -> usize {
    let mut floor = floor_map_minimal(lines);
    let mut count = 0;

    for l in lines.iter() {
        count += draw_line_axis_diag(&mut floor, l);
    }

    count
}

fn draw_line_axis_diag(floor: &mut [Vec<u8>], l: &Line) -> usize {
    let mut overlaps = 0;
    for coord in DrawLine::new(l) {
        let x = coord.x as usize;
        let y = coord.y as usize;
        floor[y][x] += 1;
        if floor[y][x] == 2 {
            overlaps += 1
        }
    }
    overlaps
}

struct DrawLine {
    end: Point,
    coord: Point,
    inc_x: i8,
    inc_y: i8,
    done: bool,
}

impl DrawLine {
    fn new(line: &Line) -> DrawLine {
        fn inc(x1: u16, x2: u16) -> i8 {
            match x1.cmp(&x2) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => 0,
            }
        }
        let inc_x = inc(line[0].x, line[1].x);
        let inc_y = inc(line[0].y, line[1].y);
        DrawLine {
            coord: line[0],
            end: line[1],
            done: false,
            inc_x,
            inc_y,
        }
    }
}

impl Iterator for DrawLine {
    type Item = Point;

    /*
     * We want to make sure to send the first element as well as the last one.
     *
     * we also don't want to do any operation on the last one to prevent any kind of over/underflow
     */
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let res = self.coord;
        if self.coord == self.end {
            self.done = true;
            return Some(res);
        }
        self.coord.x = (self.coord.x as i32 + self.inc_x as i32) as u16;
        self.coord.y = (self.coord.y as i32 + self.inc_y as i32) as u16;
        Some(res)
    }
}

#[test]
fn test() {
    let lines = parse(include_str!("../sample.txt"));
    //part 1
    let points = axes_lines_overlap(&lines);
    assert_eq!(points, 5);
    let points = lines_overlap(&lines);
    assert_eq!(points, 12);
}
