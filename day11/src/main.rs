fn main() {
    let octopuses = parse(include_str!("../input.txt"));
    //part 1
    let flashes = simulateflashes(&octopuses, 100);
    println!("Number of flashes in 100 steps: {}", flashes);
    //part 2
    let step = simulate_until_sync(&octopuses);
    println!("Sync step: {}", step);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|x| {
                    let x = x - b'0';
                    assert!(x < 10);
                    x
                })
                .collect()
        })
        .collect()
}

fn simulateflashes(octopuses: &[Vec<u8>], steps: usize) -> usize {
    let mut count = 0;
    // copy to work
    let mut octopuses: Vec<Vec<u8>> = octopuses
        .iter()
        .map(|l| l.iter().copied().collect())
        .collect();
    for _ in 0..steps {
        count += step(&mut *octopuses);
    }
    count
}

struct PointAdjascentIterator {
    x: usize,
    y: usize,
    xmax: usize,
    ymax: usize,
    cur: usize,
}

impl PointAdjascentIterator {
    fn new<T>(x: usize, y: usize, map: &[Vec<T>]) -> PointAdjascentIterator {
        PointAdjascentIterator {
            x,
            y,
            xmax: map[0].len(),
            ymax: map.len(),
            cur: 0,
        }
    }
    fn valid(&self, x: i32, y: i32) -> bool {
        if x as usize == self.x && y as usize == self.y {
            return false;
        }
        if x < 0 || y < 0 || x as usize >= self.xmax || y as usize >= self.ymax {
            return false;
        }
        true
    }
}

impl Iterator for PointAdjascentIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur < 9 {
            let x = self.x as i32 + self.cur as i32 % 3 - 1;
            let y = self.y as i32 + self.cur as i32 / 3 - 1;
            self.cur += 1;
            if self.valid(x, y) {
                return Some((x as usize, y as usize));
            }
        }
        None
    }
}

fn step(octopuses: &mut [Vec<u8>]) -> usize {
    let mut flashes = 0;
    let mut workqueue: Vec<(usize, usize)> = Vec::new();
    let mut flashmap = vec![vec![false; octopuses[0].len()]; octopuses.len()];
    // First increment
    for (y, l) in octopuses.iter_mut().enumerate() {
        for (x, o) in l.iter_mut().enumerate() {
            *o += 1;
            if *o > 9 {
                workqueue.push((x, y));
            }
        }
    }
    while let Some(p) = workqueue.pop() {
        let x = p.0;
        let y = p.1;
        if flashmap[y][x] {
            continue;
        }
        flashmap[y][x] = true;
        flashes += 1;
        for opos in PointAdjascentIterator::new(x, y, octopuses) {
            let x = opos.0;
            let y = opos.1;
            octopuses[y][x] += 1;
            if octopuses[y][x] > 9 && !flashmap[y][x] {
                workqueue.push((x, y));
            }
        }
    }
    for (y, l) in flashmap.iter().enumerate() {
        for (x, f) in l.iter().enumerate() {
            if *f {
                octopuses[y][x] = 0;
            }
        }
    }
    flashes
}

fn simulate_until_sync(octopuses: &[Vec<u8>]) -> usize {
    let mut count = 0;
    // copy to work
    let mut octopuses: Vec<Vec<u8>> = octopuses
        .iter()
        .map(|l| l.iter().copied().collect())
        .collect();
    loop {
        count += 1;
        step(&mut *octopuses);
        if in_sync(&octopuses) {
            return count;
        }
    }
}

fn in_sync(octopuses: &[Vec<u8>]) -> bool {
    let val = octopuses[0][0];
    for l in octopuses.iter() {
        for o in l.iter() {
            if *o != val {
                return false;
            }
        }
    }
    true
}

#[test]
fn test() {
    let octopuses = parse(include_str!("../sample.txt"));
    //part 1
    let flashes = simulateflashes(&octopuses, 100);
    assert_eq!(flashes, 1656);
    //part 2
    let syncstep = simulate_until_sync(&octopuses);
    assert_eq!(syncstep, 195);
}
