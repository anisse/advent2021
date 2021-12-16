type BitVec = Vec<bool>;
type BitVecRef<'a> = &'a [bool];

fn main() {
    let transmission = parse(include_str!("../input.txt"));
    //part 1
    let versum = decode_versions(&transmission);
    println!("Sum of packet versions : {}", versum);
    //part 2
    let result = eval_expressions(&transmission);
    println!("Evaluation: {}", result);
}
fn parse(input: &str) -> BitVec {
    input
        .trim()
        .chars()
        .map(|c| {
            let n = usize::from_str_radix(&c.to_string(), 16).expect("hex digit");
            (0..4)
                .map(|i| ((n >> (3 - i)) & 1) != 0)
                .collect::<Vec<bool>>()
        })
        .flatten()
        .collect()
}

fn consume(b: &mut BitVec, bits: usize) -> usize {
    if bits >= usize::BITS as usize {
        panic!("too many bits to consume");
    }
    if b.len() < bits {
        panic!("not enough remaining bits");
    }
    b.drain(0..bits).fold(0, |sum, n| (sum << 1) | (n as usize))
}

fn print_bitvec(data: BitVecRef, bits: usize) {
    data.iter().take(bits).for_each(|b| print!("{}", *b as u8));
}

fn decode_versions(transmission: BitVecRef) -> usize {
    let mut total = 0;
    let mut b: BitVec = transmission.to_vec();
    while b.len() > 8 {
        total += decode_packet(&mut b)
    }
    total
}

fn decode_packet(data: &mut BitVec) -> usize {
    let mut version = consume(data, 3);

    let packet_type = consume(data, 3);
    match packet_type {
        //type
        4 => {
            // literal
            while (consume(data, 5) & 0x10) != 0 {}
        }
        _ => {
            // operator
            let length_type = consume(data, 1);
            match length_type {
                // length type ID
                0 => {
                    let total = consume(data, 15);
                    let current = data.len();
                    while data.len() != (current - total) {
                        version += decode_packet(data);
                    }
                }
                1 => {
                    let mut length = consume(data, 11);
                    while length > 0 {
                        version += decode_packet(data);
                        length -= 1;
                    }
                }
                _ => {
                    print_bitvec(data, 32);
                    unreachable!();
                }
            };
        }
    }

    version
}

enum Expression {
    Sum(Vec<Expression>),
    Product(Vec<Expression>),
    Minimum(Vec<Expression>),
    Maximum(Vec<Expression>),
    Greater((Box<Expression>, Box<Expression>)),
    Less((Box<Expression>, Box<Expression>)),
    Equal((Box<Expression>, Box<Expression>)),
    Literal(usize),
}

impl Expression {
    fn eval(&self) -> usize {
        match self {
            Expression::Sum(e) => e.iter().map(|x| x.eval()).sum(),
            Expression::Product(e) => e.iter().map(|x| x.eval()).reduce(|x, y| x * y).unwrap(),
            Expression::Minimum(e) => e.iter().map(|x| x.eval()).min().unwrap(),
            Expression::Maximum(e) => e.iter().map(|x| x.eval()).max().unwrap(),
            Expression::Greater((a, b)) => (a.eval() > b.eval()) as usize,
            Expression::Less((a, b)) => (a.eval() < b.eval()) as usize,
            Expression::Equal((a, b)) => (a.eval() == b.eval()) as usize,
            Expression::Literal(a) => *a,
        }
    }
}

fn eval_expressions(transmission: BitVecRef) -> usize {
    let mut b: BitVec = transmission.to_vec();
    let e = parse_packet(&mut b);
    e.eval()
}

fn decode_operator(data: &mut BitVec) -> Vec<Expression> {
    let mut exp: Vec<Expression> = Vec::new();
    // operator
    let length_type = consume(data, 1);
    match length_type {
        // length type ID
        0 => {
            let total = consume(data, 15);
            let current = data.len();
            while data.len() != (current - total) {
                exp.push(parse_packet(data));
            }
        }
        1 => {
            let mut length = consume(data, 11);
            while length > 0 {
                exp.push(parse_packet(data));
                length -= 1;
            }
        }
        _ => {
            print_bitvec(data, 32);
            unreachable!();
        }
    }
    exp
}

fn parse_packet(data: &mut BitVec) -> Expression {
    consume(data, 3);

    let packet_type = consume(data, 3);
    match packet_type {
        4 => {
            // literal
            let mut nibble = consume(data, 5);
            let mut l = nibble & 0xF;
            while (nibble & 0x10) != 0 {
                nibble = consume(data, 5);
                l = (l << 4) | (nibble & 0xF);
            }
            Expression::Literal(l)
        }
        0 => Expression::Sum(decode_operator(data)),
        1 => Expression::Product(decode_operator(data)),
        2 => Expression::Minimum(decode_operator(data)),
        3 => Expression::Maximum(decode_operator(data)),
        5 | 6 | 7 => {
            let mut sub_exp = decode_operator(data);
            assert_eq!(sub_exp.len(), 2);
            let x2 = Box::new(sub_exp.pop().unwrap());
            let x1 = Box::new(sub_exp.pop().unwrap());
            match packet_type {
                5 => Expression::Greater((x1, x2)),
                6 => Expression::Less((x1, x2)),
                7 => Expression::Equal((x1, x2)),
                _ => unreachable!(),
            }
        }
        _ => {
            print_bitvec(data, 32);
            panic!(
                "Unknown packet type {} (remaining: {})",
                packet_type,
                data.len()
            );
        }
    }
}

#[test]
fn test() {
    //part 1
    assert_eq!(parse("D"), &[true, true, false, true]);
    assert_eq!(decode_versions(&parse("D2FE28")), 6);
    assert_eq!(decode_versions(&parse("8A004A801A8002F478")), 16);
    assert_eq!(decode_versions(&parse("620080001611562C8802118E34")), 12);
    assert_eq!(decode_versions(&parse("C0015000016115A2E0802F182340")), 23);
    assert_eq!(
        decode_versions(&parse("A0016C880162017C3686B18A3D4780")),
        31
    );
    //part 2
    assert_eq!(eval_expressions(&parse("C200B40A82")), 3);
    assert_eq!(eval_expressions(&parse("04005AC33890")), 54);
    assert_eq!(eval_expressions(&parse("880086C3E88112")), 7);
    assert_eq!(eval_expressions(&parse("CE00C43D881120")), 9);
    assert_eq!(eval_expressions(&parse("D8005AC2A8F0")), 1);
    assert_eq!(eval_expressions(&parse("F600BC2D8F")), 0);
    assert_eq!(eval_expressions(&parse("9C005AC2F8F0")), 0);
    assert_eq!(eval_expressions(&parse("9C0141080250320F1802104A08")), 1);
}
