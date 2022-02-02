use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Option<T>>>>;
type Lin<T> = Rc<RefCell<Option<T>>>;

#[derive(Debug)]
struct Num {
    val: u8,
    lneigh: Link<Node>,
    rneigh: Link<Node>,
}
impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
enum Node {
    Leaf(Num),
    Pair { l: Lin<Node>, r: Lin<Node> },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Node::Leaf(num) => write!(f, "{}", num),
            Node::Pair { l, r } => write!(
                f,
                "[{},{}]",
                &*(*l).borrow().as_ref().ok_or(std::fmt::Error)?,
                &*(*r).borrow().as_ref().ok_or(std::fmt::Error)?,
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
impl Display for PLin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", (*self.0).borrow().as_ref().ok_or(std::fmt::Error)?)
    }
}

enum Dir {
    Left,
    Right,
}

struct LeafIter {
    l: Link<Node>,
    dir: Dir,
}

impl Iterator for LeafIter {
    type Item = Lin<Node>;

    fn next(&mut self) -> Link<Node> {
        match self.l.take() {
            Some(n) => {
                {
                    let x = (*n).borrow();
                    if let Node::Leaf(num) = x.as_ref().expect("not a leaf") {
                        let next = match self.dir {
                            Dir::Left => &num.lneigh,
                            Dir::Right => &num.rneigh,
                        };
                        if let Some(l) = next {
                            self.l.replace(Rc::clone(l));
                        }
                    }
                }
                Some(n)
            }
            None => None,
        }
    }
}

fn iter_leafs(l: &Link<Node>, dir: Dir) -> LeafIter {
    iter_num(&edge_leaf(l, Dir::Left), dir)
}

fn iter_num(l: &Link<Node>, dir: Dir) -> LeafIter {
    // TODO: assert we have a leaf
    LeafIter {
        l: l.as_ref().map(Rc::clone),
        dir,
    }
}

fn edge_leaf(l: &Link<Node>, dir: Dir) -> Link<Node> {
    let mut current = l;
    let mut tmp;
    while let Some(n) = current {
        let x = Rc::clone(n);
        if let Some(n) = &*(*x).borrow() {
            match n {
                Node::Leaf(_) => return current.as_ref().map(Rc::clone),
                Node::Pair { l, r } => {
                    tmp = Some(Rc::clone(match dir {
                        Dir::Left => l,
                        Dir::Right => r,
                    }));
                    current = &tmp;
                }
            };
        };
    }
    None
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
fn replace_neigh(node: &Link<Node>, new_neigh: &Link<Node>, dir: Dir) {
    if let (Some(n), Some(new)) = (node, new_neigh) {
        if let Some(Node::Leaf(ref mut num)) = &mut *(*n).borrow_mut() {
            match dir {
                Dir::Left => {
                    num.lneigh.replace(Rc::clone(new));
                }
                Dir::Right => {
                    num.rneigh.replace(Rc::clone(new));
                }
            }
        }
    }
}

fn redo_rneigh(left: &Link<Node>) {
    let mut right = None;
    for l in iter_num(left, Dir::Left) {
        replace_neigh(&Some(Rc::clone(&l)), &right, Dir::Right);
        right = Some(Rc::clone(&l));
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
    (Rc::new(RefCell::new(Some(Node::Pair { l, r }))), left, pos)
}
fn parse_node(input: &str, left: Link<Node>) -> (Lin<Node>, Link<Node>, usize) {
    match input.chars().next().expect("missing char") {
        '[' => parse_tree(input, left),
        '0'..='9' => {
            let (l, l1) = parse_literal(input, left);
            let n = Rc::new(RefCell::new(Some(Node::Leaf(l))));
            let ln = Rc::clone(&n);
            (n, Some(ln), l1)
        }
        _ => panic!("Unexpected token"),
    }
}
fn parse_literal(input: &str, left: Link<Node>) -> (Num, usize) {
    let s = input
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    (
        Num {
            val: s.parse::<u8>().expect("not int"),
            lneigh: left,
            rneigh: None,
        },
        s.len(),
    )
}
fn addition_magnitude(pairs: Vec<Lin<Node>>) -> usize {
    magnitude(add_list(pairs))
}
fn add_list(pairs: Vec<Lin<Node>>) -> Lin<Node> {
    pairs
        .into_iter()
        .reduce(|a: Lin<Node>, b: Lin<Node>| -> _ { add(a, b) })
        .expect("no reduction")
}

fn add(l: Lin<Node>, r: Lin<Node>) -> Lin<Node> {
    let lr = edge_leaf(&Some(Rc::clone(&l)), Dir::Right);
    let rl = edge_leaf(&Some(Rc::clone(&r)), Dir::Left);

    replace_neigh(&lr, &rl, Dir::Right);
    replace_neigh(&rl, &lr, Dir::Left);

    reduce(Rc::new(RefCell::new(Some(Node::Pair { l, r }))))
}

fn reduce(t: Lin<Node>) -> Lin<Node> {
    //println!("Reducing : {}", PLin::new(&t));
    while explode(&t) || split(&t) {
        //println!("Cycle    : {}", PLin::new(&t));
    }
    //println!("Done     : {}", PLin::new(&t));
    t
}

fn split(t: &Lin<Node>) -> bool {
    //println!("Splitting: {}", PLin::new(t));
    for l in iter_leafs(&Some(Rc::clone(t)), Dir::Right) {
        let node = &mut *(*l).borrow_mut();
        if let Some(Node::Leaf(num)) = node {
            if num.val >= 10 {
                // replace with a split
                let right = Rc::new(RefCell::new(None));
                let left = Rc::new(RefCell::new(None));
                replace_neigh(&num.lneigh, &Some(Rc::clone(&left)), Dir::Right);
                replace_neigh(&num.rneigh, &Some(Rc::clone(&right)), Dir::Left);
                (*left).borrow_mut().replace(Node::Leaf(Num {
                    val: num.val / 2,
                    lneigh: num.lneigh.take(),
                    rneigh: Some(Rc::clone(&right)),
                }));
                (*right).borrow_mut().replace(Node::Leaf(Num {
                    val: num.val - num.val / 2,
                    lneigh: Some(Rc::clone(&left)),
                    rneigh: num.rneigh.take(),
                }));
                let new = Node::Pair { l: left, r: right };
                node.replace(new);
                //if let Some(Node::Leaf(num)) = &*(*left).borrow_mut() {}
                //println!("After split: {}", PLin::new(t));
                return true;
            }
        }
    }
    false
}

fn explode(t: &Lin<Node>) -> bool {
    //println!("Exploding: {}", PLin::new(t));
    let mut queue: Vec<(Lin<Node>, usize)> = vec![(Rc::clone(t), 1)];

    while let Some((n, depth)) = queue.pop() {
        let nmod = &mut *(*n).borrow_mut();
        if let Some(Node::Pair { l, r }) = nmod {
            //let mut rvalue: Option<u8> = None;
            let mut op = |nod: &Lin<Node>| match &*(*nod).borrow() {
                Some(Node::Pair { l: _, r: _ }) => {
                    queue.insert(queue.len(), (Rc::clone(nod), depth + 1));
                    None
                }
                Some(Node::Leaf(num)) => {
                    //dbg!(num.val);
                    Some(Num {
                        val: num.val,
                        lneigh: num.lneigh.as_ref().map(Rc::clone),
                        rneigh: num.rneigh.as_ref().map(Rc::clone),
                    })
                }
                _ => None,
            };
            let rvalue = op(r);
            let lvalue = op(l);
            //println!("At depth {}", depth);
            if depth > 4 {
                if let (Some(lval), Some(rval)) = (lvalue, rvalue) {
                    //println!("Taking care of values [{}, {}]", lval.val, rval.val);
                    (*nmod).replace(Node::Leaf(Num {
                        val: 0,
                        lneigh: lval.lneigh.as_ref().map(Rc::clone),
                        rneigh: rval.rneigh.as_ref().map(Rc::clone),
                    }));
                    if let Some(ref left) = lval.lneigh {
                        if let Some(Node::Leaf(leftnum)) = &mut *(*left).borrow_mut() {
                            leftnum.val += lval.val;
                            leftnum.rneigh = Some(Rc::clone(&n));
                        }
                    }
                    if let Some(ref right) = rval.rneigh {
                        if let Some(Node::Leaf(rightnum)) = &mut *(*right).borrow_mut() {
                            rightnum.val += rval.val;
                            rightnum.lneigh = Some(Rc::clone(&n));
                        }
                    }
                    /*
                    if let Some(node) = &*nmod {
                        println!("We have {}", node);
                    }
                    */
                    return true;
                }
            }
        }
    }

    false
}

fn magnitude(t: Lin<Node>) -> usize {
    match &*(*t).borrow() {
        Some(Node::Pair { l, r }) => 3 * magnitude(Rc::clone(l)) + 2 * magnitude(Rc::clone(r)),
        Some(Node::Leaf(num)) => num.val as usize,
        _ => unreachable!(),
    }
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
        Some(Node::Pair { l, r: _ }) => {
            let mut values: Vec<u8> = Vec::new();
            let v = Some(Rc::clone(l));
            for l in iter_leafs(&v, Dir::Right) {
                if let Some(Node::Leaf(num)) = &*(*l).borrow() {
                    values.push(num.val);
                }
            }
            assert_eq!(
                vec![1, 1, 3, 5, 3, 1, 3, 8, 7, 4, 9, 6, 9, 8, 2, 7, 3],
                values
            );
        }
        _ => panic!("not pair"),
    };
}
#[test]
fn test_explode() {
    let in_out = vec![
        (
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        ),
        (
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        ),
        (
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ),
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
        (
            "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]",
            "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]",
        ),
        (
            "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        ),
    ];
    for (inp, outp) in in_out {
        dbg!(inp);
        let tmp = parse(inp);
        let x = tmp.first().unwrap();
        explode(x);
        assert_eq!(format!("{}", PLin::new(x)), outp);
        //dbg!(outp);
    }
}
#[test]
fn test_split() {
    let in_out = vec![
        (
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        ),
        (
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        ),
    ];
    for (inp, outp) in in_out {
        let tmp = parse(inp);
        let x = tmp.first().unwrap();
        split(x);
        assert_eq!(format!("{}", PLin::new(x)), outp);
    }
}

#[test]
fn test_add() {
    let in_out = vec![
        (
            "[1,1]
[2,2]
[3,3]
[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        ),
        (
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        ),
        (
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ),
        (
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        ),
        (
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ),
        (
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        ),
    ];
    for (inp, outp) in in_out {
        let pairs = parse(inp);
        let res = add_list(pairs);
        assert_eq!(format!("{}", PLin::new(&res)), outp);
    }
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
