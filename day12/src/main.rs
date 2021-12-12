use std::collections::{HashMap, HashSet};

type Connection<'a> = [&'a str; 2];

type Graph<'a, 'b> = HashMap<&'b str, Vec<&'a str>>;

fn main() {
    let map = parse(include_str!("../input.txt"));
    //part 1
    let path_count = count_paths(&map);
    println!("Number of paths: {}", path_count);
    //part 2
    let path_count = count_paths_2(&map);
    println!("Number of paths 2: {}", path_count);
}
fn parse(input: &str) -> Vec<Connection> {
    input
        .lines()
        .map(|l| {
            l.split('-')
                .collect::<Vec<&str>>()
                .try_into()
                .expect("wrong number of elements")
        })
        .collect()
}
fn count_paths(map: &[Connection]) -> usize {
    let g = build_graph(map);
    let mut p = vec!["start".to_string()];
    find_paths(&g, &mut p, check_small_caves)
}

fn find_paths(g: &Graph, prev: &mut Vec<String>, check_caves: fn(&[String]) -> bool) -> usize {
    let cur = prev.last().expect("no last element");
    let v = g
        .get(cur as &str)
        .unwrap_or_else(|| panic!("{} has no connections", cur));
    let mut count = 0;
    for next in v.iter() {
        if *next == "end" {
            count += 1;
            continue;
        }
        let x: String = (**next).to_owned().clone();
        prev.push(x);
        if check_caves(prev) {
            count += find_paths(g, prev, check_caves);
        }
        prev.pop();
    }
    count
}
fn check_small_caves(p: &[String]) -> bool {
    let mut small: HashMap<String, bool> = HashMap::new();
    for cave in p.iter() {
        if is_lower(cave) {
            if small.get(cave).is_some() {
                return false;
            }
            small.insert(cave.clone(), true);
        }
    }
    true
}

fn is_lower(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn build_graph<'a>(map: &[Connection<'a>]) -> Graph<'a, 'a> {
    let mut g = Graph::new();
    for c in map.iter() {
        if let Some(v) = g.get_mut(c[0]) {
            v.push(c[1]);
        } else {
            g.insert(c[0], vec![c[1]]);
        }
        if let Some(v) = g.get_mut(c[1]) {
            v.push(c[0]);
        } else {
            g.insert(c[1], vec![c[0]]);
        }
    }
    g
}
fn count_paths_2(map: &[Connection]) -> usize {
    let g = build_graph(map);
    let mut p = vec!["start".to_string()];
    find_paths(&g, &mut p, check_small_caves_2)
}

fn check_small_caves_2(p: &[String]) -> bool {
    let mut small: HashSet<String> = HashSet::new();
    let mut single_small_cave = false;
    for cave in p.iter().skip(1) {
        if cave == "start" {
            return false;
        }
        if is_lower(cave) {
            if small.get(cave).is_some() {
                if single_small_cave {
                    return false;
                }
                single_small_cave = true;
            } else {
                small.insert(cave.clone());
            }
        }
    }
    true
}

#[test]
fn test() {
    let map = parse(include_str!("../sample.txt"));
    let map2 = parse(include_str!("../sample2.txt"));
    let map3 = parse(include_str!("../sample3.txt"));
    //part 1
    let path_count = count_paths(&map);
    assert_eq!(path_count, 10);
    //part 1 example 2
    let path_count = count_paths(&map2);
    assert_eq!(path_count, 19);
    //part 1 example 3
    let path_count = count_paths(&map3);
    assert_eq!(path_count, 226);
    //part 2
    let path_count = count_paths_2(&map);
    assert_eq!(path_count, 36);
    //part 2 example 2
    let path_count = count_paths_2(&map2);
    assert_eq!(path_count, 103);
    //part 2 example 3
    let path_count = count_paths_2(&map3);
    assert_eq!(path_count, 3509);
}
