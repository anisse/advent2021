use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<T>>>;
type Lin<T> = Rc<RefCell<T>>;

#[derive(Debug)]
struct Num {
    val: u8,
    lneigh: Link<Num>,
    rneigh: Link<Num>,
}
impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
enum Node {
    Leaf(Link<Num>),
    Pair { l: Link<Node>, r: Link<Node> },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Node::Leaf(num) => write!(f, "{}", (**num.as_ref().unwrap()).borrow()),
            Node::Pair { l, r } => write!(
                f,
                "[{},{}]",
                (**l.as_ref().unwrap()).borrow(),
                RefCell::borrow(r.as_ref().unwrap())
            ),
        }
    }
}
struct PLin(Lin<Node>);

impl PLin {
    fn new(l: &Lin<Node>) -> Self {
        PLin(Rc::clone(l))
    }
}

enum Dir {
    Left,
    Right,
}

struct LeafIter {
    l: Link<Num>,
    ldir: Dir,
}

impl Iterator for LeafIter {
    type Item = Lin<Num>;

    fn next(&mut self) -> Link<Num> {
        match self.l {
            Some(n) => {
                let next = match self.ldir {
                    Dir::Left => (*n).borrow().lneigh,
                    Dir::Right => (*n).borrow().rneigh,
                };
                match next {
                    Some(l) => self.l.replace(l),
                    None => self.l.take(),
                }
            }
            None => None,
        }
    }
}

impl Display for PLin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", RefCell::borrow(&*(self.0)))
    }
}
fn main() {
    let pairs = parse(include_str!("../input.txt"));
    //part 1
    let magnitude = addition_magnitude(pairs);
    println!("Magnitude: {}", magnitude);
    //part 2
    //let magnitude = addition_magniture2(&pairs);
    //println!("Magnitude2: {}", magnitude);
}
fn parse(input: &str) -> Vec<Lin<Node>> {
    input
        .lines()
        .map(|l| {
            let (tree, _, len) = parse_tree(l, None);
            assert_eq!(len, l.len());
            tree
        })
        .collect()
}

fn redo_rneigh(left: &Link<Node>) {
    let mut right = None;
    let n;
    if let Some(ref nr) = left {
        n = Rc::clone(nr);
    } else {
        return;
    }
    let mut left = Some(n);
    while let Some(ref n) = left {
        let n = Rc::clone(n);
        let mut x = n.borrow_mut();
        if let Node::Leaf {
            val: _,
            ref lneigh,
            ref mut rneigh,
        } = *x
        {
            if rneigh.is_some() {
                // Reached already done section
                break;
            }
            if let Some(r) = right {
                rneigh.replace(r);
            }
            right = Some(Rc::clone(&n));
            if let Some(l) = lneigh {
                left = Some(Rc::clone(l));
            } else {
                break;
            }
        }
    }
}

fn parse_tree(input: &str, left: Link<Node>) -> (Lin<Node>, Link<Node>, usize) {
    let mut pos;
    assert_eq!(input.chars().next(), Some('['));
    pos = 1;
    let (l, left, l1) = parse_node(&input[pos..], left);
    pos += l1;
    assert_eq!(input.chars().nth(pos), Some(','));
    pos += 1;
    let (r, left, l2) = parse_node(&input[pos..], left);
    pos += l2;
    assert_eq!(input.chars().nth(pos), Some(']'));
    pos += 1;
    redo_rneigh(&left);
    (
        Rc::new(RefCell::new(Node::Pair {
            l: Some(l),
            r: Some(r),
        })),
        left,
        pos,
    )
}
fn parse_node(input: &str, left: Link<Node>) -> (Lin<Node>, Link<Node>, usize) {
    match input.chars().next().unwrap() {
        '[' => parse_tree(input, left),
        '0'..='9' => {
            let (l, l1) = parse_literal(input, left);
            (Rc::clone(&l), Some(l), l1)
        }
        _ => panic!("Unexpected token"),
    }
}
fn parse_literal(input: &str, left: Link<Node>) -> (Lin<Node>, usize) {
    let s = input
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    (
        Rc::new(RefCell::new(Node::Leaf {
            val: s.parse::<u8>().expect("not int"),
            lneigh: left,
            rneigh: None,
        })),
        s.len(),
    )
}
fn addition_magnitude(pairs: Vec<Lin<Node>>) -> usize {
    let res = pairs
        .into_iter()
        .reduce(|a: Lin<Node>, b: Lin<Node>| -> _ { add(a, b) })
        .expect("no reduction");
    magnitude(res)
}

fn add(l: Lin<Node>, r: Lin<Node>) -> Lin<Node> {
    Rc::new(RefCell::new(Node::Pair {
        l: Some(l),
        r: Some(r),
    }))
}

fn reduce(mut t: Lin<Node>) -> Lin<Node> {
    let mut change = true;
    while change {
        change = false;
        if let Some(x) = explode(Rc::clone(&t)) {
            t = x;
            change = true;
        }
        if let Some(x) = split(Rc::clone(&t)) {
            t = x;
            change = true;
        }
    }
    t
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn split(t: Lin<Node>) -> Link<Node> {
    /*
    let mut result = t.clone();
    let mut next: Vec<&mut Box<Node>> = vec![&mut result];
    println!("{}", t);
    while let Some(n) = next.pop() {
        println!("{}", &*n);
        match &*n.as_mut() {
            Node::Leaf(v) => {
                if *v > 10 {
                    //println!("{:p}", t);
                    //println!("{:p}", n);
                    //dbg!(v);
                    //dbg!(&n);
                    //print_type_of(&n);
                    //print_type_of(&*n);
                    //
                    // *n = Node::Pair {
                    //    l: Box::new(Node::Leaf(*v / 2)),
                    //    r: Box::new(Node::Leaf(*v / 2 + *v % 2)),
                    //};
                    //
                    n.replace(Node::Pair {
                        l: Box::new(Node::Leaf(v / 2)),
                        r: Box::new(Node::Leaf(v / 2 + v % 2)),
                    });
                    //dbg!(&n);
                    //dbg!(t);
                    return Some(result);
                }
            }
            Node::Pair { mut l, mut r } => {
                next.push(&mut r);
                next.push(&mut l);
            }
        };
    }
    */
    None
}

fn explode(t: Lin<Node>) -> Link<Node> {
    None
}

fn magnitude(t: Lin<Node>) -> usize {
    0
}
#[test]
fn test_parse() {
    let x = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
    assert_eq!(format!("{}", PLin::new(parse(x).first().unwrap())), x);
}

#[test]
fn test_neigh_chain() {
    let x = "[1,[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]]";
    let tmp = parse(x);
    let f = tmp.first().unwrap();
    match &*f.borrow() {
        Node::Pair { l, r: _ } => match &*RefCell::borrow(l.as_ref().unwrap()) {
            Node::Leaf {
                val,
                lneigh,
                rneigh,
            } => {
                assert!(lneigh.is_none());
                let mut values: Vec<u8> = vec![*val];
                let r;
                if let Some(ref rn) = rneigh {
                    r = Rc::clone(rn);
                } else {
                    panic!("no rneigh");
                }
                let mut r = Some(r);
                while let Some(next) = r {
                    let n = next.borrow();
                    if let Node::Leaf {
                        ref val,
                        lneigh: _,
                        ref rneigh,
                    } = &*n
                    {
                        values.push(*val);
                        if let Some(rn) = rneigh {
                            r = Some(Rc::clone(rn));
                        } else {
                            break;
                        }
                    } else {
                        panic!("rneigh not leaf");
                    }
                }
                assert_eq!(
                    vec![1, 1, 3, 5, 3, 1, 3, 8, 7, 4, 9, 6, 9, 8, 2, 7, 3],
                    values
                );
            }
            _ => panic!("not leaf"),
        },
        _ => panic!("not pair"),
    };
}
#[test]
fn test_explode() {
    let tmp = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    let x = tmp.first().unwrap();
    let s = format!("{}", PLin::new(x));
    //assert!(explode(x));
    assert_eq!(s, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
}
#[test]
fn test_split() {
    let tmp = parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    let x = tmp.first().unwrap();
    //assert!(split(&mut x2));
    assert_eq!(
        format!("{}", PLin::new(x)),
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
    );
}

#[test]
fn test() {
    let pairs = parse(include_str!("../sample.txt"));
    //part 1
    let magnitude = addition_magnitude(pairs);
    assert_eq!(magnitude, 4140);
    //part 2
    // let magnitude = addition_magniture2(&pairs);
    // assert_eq!(magnitude, 42);
}
