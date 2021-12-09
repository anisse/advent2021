fn main() {
    let heightmap = parse(include_str!("../input.txt"));
    //part 1
    let risk = lowpointsrisk(&heightmap);
    println!("Risk sum: {}", risk);
    //part 2
    let basins = largest_basins(&heightmap);
    println!("Basins: {}", basins);
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

fn largest_basins(heightmap: &[Vec<u8>]) -> usize {
    let mut basin_map: Vec<Vec<bool>> = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    let mut basins_sizes: Vec<usize> = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if let Some(size) = find_basin(heightmap, &mut basin_map, x, y) {
                basins_sizes.push(size);
            }
        }
    }

    basins_sizes.sort_unstable();
    let l = basins_sizes.len();
    basins_sizes[l - 1] * basins_sizes[l - 2] * basins_sizes[l - 3]
}
fn find_basin(
    heightmap: &[Vec<u8>],
    basin_map: &mut [Vec<bool>],
    x: usize,
    y: usize,
) -> Option<usize> {
    let mut workqueue = vec![(x, y)];
    let mut current_size = 0;
    while let Some(p) = workqueue.pop() {
        let x = p.0;
        let y = p.1;
        let val = heightmap[y][x];
        if val == 9 || basin_map[y][x] {
            continue;
        }
        /* valid point */
        current_size += 1;
        basin_map[y][x] = true;

        /* next points */
        if y > 0 {
            workqueue.push((x, y - 1));
        }
        if x > 0 {
            workqueue.push((x - 1, y));
        }
        if y < heightmap.len() - 1 {
            workqueue.push((x, y + 1));
        }
        if x < heightmap[y].len() - 1 {
            workqueue.push((x + 1, y));
        }
    }
    if current_size > 0 {
        return Some(current_size);
    }
    None
}

#[test]
fn test() {
    let heightmap = parse(include_str!("../sample.txt"));
    //part 1
    let risk = lowpointsrisk(&heightmap);
    assert_eq!(risk, 15);
    //part 2
    let b = largest_basins(&heightmap);
    assert_eq!(b, 1134);
}
