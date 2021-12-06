#[derive(Debug, Clone)]
struct Fish {
    timer: u8,
}
fn main() {
    let fish_school = parse(include_str!("../input.txt"));
    // part 1
    let fish_count = simulate_fish(&fish_school, 80);
    println!("Fish after 80 days: {}", fish_count);
}
fn parse(input: &str) -> Vec<Fish> {
    input
        .trim_end()
        .split(',')
        .map(|x| Fish {
            timer: x.parse().expect("not int {}"),
        })
        .collect()
}
fn simulate_fish(school: &[Fish], days: u8) -> usize {
    let mut school = school.to_vec();
    for _ in 0..days {
        let mut new = 0;
        for f in school.iter_mut() {
            match f.timer {
                0 => {
                    f.timer = 6;
                    new += 1
                }
                _ => f.timer -= 1,
            }
        }
        for _ in 0..new {
            school.push(Fish { timer: 8 });
        }
    }
    school.len()
}

#[test]
fn test() {
    let fish_school = parse(include_str!("../sample.txt"));
    // part 1
    let fish_count = simulate_fish(&fish_school, 18);
    assert_eq!(26, fish_count);
    let fish_count = simulate_fish(&fish_school, 80);
    assert_eq!(5934, fish_count);
}
