fn main() {
    let depths = parse(include_str!("../input.txt"));
    let count = count_increases(&depths);
    //part 1
    println!("Number of increases: {}", count);
    //part 2
    let count = count_window_increases(&depths);
    println!("Number of increases with window of 3: {}", count);
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| x.parse::<usize>().expect("Not an int"))
        .collect()
}

fn count_increases(depths: &[usize]) -> usize {
    let mut inc = 0;
    for (i, d) in depths.iter().enumerate().skip(1) {
        if depths[i - 1] < *d {
            inc += 1;
        }
    }
    inc
}

struct SlidingWindow<T>
where
    T: Clone,
{
    buf: Vec<T>,
    size: usize,
}

impl<T> SlidingWindow<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: Copy,
    T: Default,
{
    fn new(size: usize) -> SlidingWindow<T> {
        SlidingWindow {
            buf: vec![Default::default(); size * 2],
            size,
        }
    }

    fn set(&mut self, val: T, offset: usize) {
        for i in offset..(offset + self.size) {
            let idx = i % (self.size * 2);
            self.buf[idx] = self.buf[idx] + val;
        }
        self.buf[(offset + self.size) % (self.size * 2)] = Default::default();
    }

    fn get(&self, offset: usize) -> T {
        self.buf[offset % (self.size * 2)]
    }
}

fn count_window_increases(depths: &[usize]) -> usize {
    const WIN_SIZE: usize = 3;
    let mut inc = 0;

    let mut win: SlidingWindow<usize> = SlidingWindow::new(WIN_SIZE);
    for (i, d) in depths.iter().enumerate() {
        win.set(*d, i);
        if i >= WIN_SIZE && win.get(i) > win.get(i - 1) {
            inc += 1;
        }
    }
    inc
}

#[test]
fn test() {
    let depths = parse(include_str!("../sample.txt"));
    let count = count_increases(&depths);
    assert_eq!(count, 7);
    let count = count_window_increases(&depths);
    assert_eq!(count, 5);
}
