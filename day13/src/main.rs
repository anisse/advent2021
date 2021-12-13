use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Dot {
    x: usize,
    y: usize,
}
#[derive(Debug)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}
fn main() {
    let (dots, instructions) = parse(include_str!("../input.txt"));
    //part 1
    let dot_count = first_fold_dots(&dots, &instructions);
    println!("Dots after first fold: {}", dot_count);
    //part 2
    let dot_map = fold_all(&dots, &instructions);
    println!("Eight capital letters:");
    debug_map(&dot_map.iter().copied().collect::<Vec<_>>());
}
fn parse(input: &str) -> (Vec<Dot>, Vec<Instruction>) {
    let mut dots = Vec::new();
    let mut ins = Vec::new();
    let mut lines = input.lines();
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let mut dot = l.split(',').map(|c| c.parse::<usize>().expect("not int"));
        dots.push(Dot {
            x: dot.next().expect("no x"),
            y: dot.next().expect("no y"),
        });
    }
    for l in lines {
        let mut s = l.split_ascii_whitespace();
        assert_eq!("fold", s.next().expect("fold"));
        assert_eq!("along", s.next().expect("along"));
        let mut fold = s.next().expect("no fold").split('=');
        let axis = fold.next().expect("axis");
        let val = fold
            .next()
            .expect("no value")
            .parse::<usize>()
            .expect("not int");
        ins.push(match axis {
            "x" => Instruction::FoldX(val),
            "y" => Instruction::FoldY(val),
            _ => unreachable!(),
        });
    }
    (dots, ins)
}

fn first_fold_dots(dots: &[Dot], instructions: &[Instruction]) -> usize {
    let mut dotset: HashSet<Dot> = dots.iter().copied().collect();

    fold(&mut dotset, &instructions[0]);
    dotset.len()
}

fn debug_map(dots: &[Dot]) {
    let max_x = dots.iter().max_by(|a, b| a.x.cmp(&b.x)).expect("max x").x;
    let max_y = dots.iter().max_by(|a, b| a.y.cmp(&b.y)).expect("max y").y;
    let mut map = vec![vec!['.'; max_x + 1]; max_y + 1];

    for d in dots.iter() {
        map[d.y][d.x] = '#';
    }
    for l in map {
        println!("{}", l.iter().collect::<String>());
    }
}

fn fold(dots: &mut HashSet<Dot>, f: &Instruction) {
    let mut removed = Vec::new();
    let mut added = Vec::new();
    for d in dots.iter() {
        match update_coord(d, f) {
            Action::Remove => removed.push(*d),
            Action::NoChange => {}
            Action::New(dnew) => {
                removed.push(*d);
                added.push(dnew);
            }
        }
    }
    for r in removed.iter() {
        dots.remove(r);
    }
    for a in added.iter() {
        dots.insert(*a);
    }
}

enum Action {
    NoChange,
    Remove,
    New(Dot),
}

fn update_coord(d: &Dot, f: &Instruction) -> Action {
    match f {
        Instruction::FoldX(x) => {
            if d.x == *x {
                return Action::Remove;
            }
            if d.x < *x {
                return Action::NoChange;
            }
            Action::New(Dot {
                y: d.y,
                x: d.x - (d.x - x) * 2,
            })
        }
        Instruction::FoldY(y) => {
            if d.y == *y {
                return Action::Remove;
            }
            if d.y < *y {
                return Action::NoChange;
            }
            Action::New(Dot {
                x: d.x,
                y: d.y - (d.y - y) * 2,
            })
        }
    }
}

fn fold_all(dots: &[Dot], instructions: &[Instruction]) -> HashSet<Dot> {
    let mut dotset: HashSet<Dot> = dots.iter().copied().collect();

    for f in instructions {
        fold(&mut dotset, f);
    }
    dotset
}

#[test]
fn test() {
    let (dots, instructions) = parse(include_str!("../sample.txt"));
    //part 1
    let dot_count = first_fold_dots(&dots, &instructions);
    assert_eq!(dot_count, 17);
}
