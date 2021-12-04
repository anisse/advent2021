fn main() {
    let diag = parse(include_str!("../input.txt"));
    //part 1
    let pow = power_consumption(&diag);
    println!("Power: {}", pow);
    //part 2
    let ls = life_support(&diag);
    println!("Life support: {}", ls);
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|bit| match bit {
                    '0' => false,
                    '1' => true,
                    _ => panic!("not 0 or 1"),
                })
                .collect()
        })
        .collect()
}
fn power_consumption(diag: &[Vec<bool>]) -> usize {
    let bitlen = diag[0].len();
    let mut count = vec![0; bitlen];
    for row in diag.iter() {
        for (i, v) in row.iter().enumerate() {
            if *v {
                count[i] += 1;
            }
        }
    }
    let gamma: usize = count
        .iter()
        .rev()
        .map(|x| if *x > (diag.len() / 2) { 1 } else { 0 })
        .enumerate()
        .map(|(i, bit)| bit << i)
        .sum();
    let bitmask = usize::MAX >> (std::mem::size_of::<usize>() * 8 - bitlen);
    let epsilon = (!gamma) & bitmask;
    gamma * epsilon
}

fn life_support(diag: &[Vec<bool>]) -> usize {
    let o2_rating = single_rating(diag, false);
    let co2_rating = single_rating(diag, true);

    o2_rating * co2_rating
}
fn single_rating(diag: &[Vec<bool>], co2: bool) -> usize {
    let mut keep = vec![true; diag.len()];
    for col in 0..diag[0].len() {
        let mut count = 0;
        let mut total = 0;
        for (i, v) in diag.iter().enumerate() {
            if keep[i] {
                total += 1;
                if v[col] {
                    count += 1;
                }
            }
        }
        let most = count >= (total - count);
        total = 0;
        let mut last = 0;
        for (i, v) in diag.iter().enumerate() {
            if keep[i] {
                keep[i] = (v[col] == most) ^ co2;
                if keep[i] {
                    total += 1;
                    last = i;
                }
            }
        }
        if total == 1 {
            return bits2int(&diag[last]);
        }
    }
    0
}

fn bits2int(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, bit)| (*bit as usize) << i)
        .fold(0, |sum, i| sum | i)
}

#[test]
fn test() {
    let diag = parse(include_str!("../sample.txt"));
    //part 1
    let pow = power_consumption(&diag);
    assert_eq!(pow, 198);
    //part 2
    let ls = life_support(&diag);
    assert_eq!(ls, 230);
}
