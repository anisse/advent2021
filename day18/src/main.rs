use std::fmt::Display;

#[derive(Debug)]
enum Node {
    Leaf(u8),
    Pair(Tree),
}
#[derive(Debug)]
struct Tree {
    left: Box<Node>,
    right: Box<Node>,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{},{}]", self.left, self.right)
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Node::Leaf(x) => write!(f, "{}", x),
            Node::Pair(t) => write!(f, "{}", t),
        }
    }
}
fn main() {
    let pairs = parse(include_str!("../input.txt"));
    //part 1
    let magnitude = addition_magniture(&pairs);
    println!("Magnitude: {}", magnitude);
    //part 2
    //let magnitude = addition_magniture2(&pairs);
    //println!("Magnitude2: {}", magnitude);
}
fn parse(input: &str) -> Vec<Tree> {
    input
        .lines()
        .map(|l| {
            let (tree, len) = parse_tree(l);
            assert_eq!(len, l.len());
            if let Node::Pair(t) = *tree {
                t
            } else {
                panic!("not tree root")
            }
        })
        .collect()
}

fn parse_tree(input: &str) -> (Box<Node>, usize) {
    let mut pos;
    assert_eq!(input.chars().next(), Some('['));
    pos = 1;
    let (left, l1) = parse_node(&input[pos..]);
    pos += l1;
    assert_eq!(input.chars().nth(pos), Some(','));
    pos += 1;
    let (right, l2) = parse_node(&input[pos..]);
    pos += l2;
    assert_eq!(input.chars().nth(pos), Some(']'));
    pos += 1;
    (Box::new(Node::Pair(Tree { left, right })), pos)
}
fn parse_node(input: &str) -> (Box<Node>, usize) {
    match input.chars().next().unwrap() {
        '[' => parse_tree(input),
        '0'..='9' => parse_literal(input),
        _ => panic!("Unexpected token"),
    }
}
fn parse_literal(input: &str) -> (Box<Node>, usize) {
    let s = input
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    (
        Box::new(Node::Leaf(s.parse::<u8>().expect("not int"))),
        s.len(),
    )
}
fn addition_magniture(pairs: &[Tree]) -> usize {
    let mut count = 0;
    for _ in pairs.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}
#[test]
fn test_parse() {
    let x = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
    assert_eq!(format!("{}", parse(x).first().unwrap()), x);
}

#[test]
fn test() {
    let pairs = parse(include_str!("../sample.txt"));
    //part 1
    let magnitude = addition_magniture(&pairs);
    assert_eq!(magnitude, 4140);
    //part 2
    // let magnitude = addition_magniture2(&pairs);
    // assert_eq!(magnitude, 42);
}
