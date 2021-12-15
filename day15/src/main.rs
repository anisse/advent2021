fn main() {
    let chiton_map = parse(include_str!("../input.txt"));
    //part 1
    let risk_level = shortest_path(&chiton_map);
    println!("Summary: {}", risk_level);
    //part 2
    //let risk_level = shortest_path2(&chiton_map);
    //println!("Summary2: {}", risk_level);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|x| x - b'0').collect())
        .collect()
}
fn shortest_path(map: &[Vec<u8>]) -> usize {
    let mut minmap = vec![vec![usize::MAX; map[0].len()]; map.len()];
    //let mut tmp = Vec::new();
    shortest_path_recur(map, 0, &mut minmap, 0, 0);
    let end_x = map[0].len() - 1;
    let end_y = map.len() - 1;
    /*
    println!(
        "{}",
        minmap
            .iter()
            .map(|l| l.iter().map(|p| format!("{:02} ", p)).collect::<String>() + "\n")
            .collect::<String>()
    );
    */
    minmap[end_y][end_x]
}
fn shortest_path_recur(
    map: &[Vec<u8>],
    total: usize,
    minmap: &mut Vec<Vec<usize>>,
    x: usize,
    y: usize,
) {
    let end_x = map[0].len() - 1;
    let end_y = map.len() - 1;
    if total >= minmap[y][x] {
        return;
    }
    minmap[y][x] = total;
    if x == end_x && y == end_y {
        return;
    }
    for i in 0..4 {
        let (x2, y2) = match i {
            0 if y > 0 => (x, y - 1),
            1 if x > 0 => (x - 1, y),
            2 if x < end_x => (x + 1, y),
            3 if y < end_y => (x, y + 1),
            _ => continue,
        };
        shortest_path_recur(map, total + map[y2][x2] as usize, minmap, x2, y2);
    }
}

#[test]
fn test() {
    let chiton_map = parse(include_str!("../sample.txt"));
    //part 1
    let risk_level = shortest_path(&chiton_map);
    assert_eq!(risk_level, 40);
    //part 2
    // let risk_level = shortest_path2(&chiton_map);
    // assert_eq!(risk_level, 42);
}
