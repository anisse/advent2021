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

fn count_window_increases(depths: &[usize]) -> usize {
    const WIN_SIZE: usize = 3;
    let mut buf = [0; WIN_SIZE * 2];
    let mut inc = 0;

    for (i, d) in depths.iter().enumerate() {
        for j in i..(WIN_SIZE + i) {
            buf[j % (WIN_SIZE * 2)] += d;
        }
        if i >= WIN_SIZE {
            // start using results and clearing
            if buf[i % (WIN_SIZE * 2)] > buf[(i - 1) % (WIN_SIZE * 2)] {
                inc += 1;
            }
            buf[(i + WIN_SIZE) % (WIN_SIZE * 2)] = 0;
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
