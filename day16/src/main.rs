type BitVec = Vec<bool>;
type BitVecRef<'a> = &'a [bool];

fn main() {
    let transmission = parse(include_str!("../input.txt"));
    //part 1
    let versum = decode_versions(&transmission);
    println!("Sum of packet versions : {}", versum);
    //part 2
    //let versum = decode_versions2(&transmission);
    //println!("Sum of packet versions 2: {}", versum);
}
fn parse(input: &str) -> BitVec {
    println!("{}", input);
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
    println!();
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
    println!("Packet start, remaining: {}", data.len());
    print_bitvec(data, 32);
    let mut version = consume(data, 3);
    println!("Version: {}", version);

    let packet_type = consume(data, 3);
    match packet_type {
        //type
        4 => {
            // literal
            print_bitvec(data, 32);
            print!("Literal .");
            while (consume(data, 5) & 0x10) != 0 {
                print!(".");
            }
            println!(" done");
        }
        _ => {
            // operator
            let length_type = consume(data, 1);
            println!("Operator with length_type {}", length_type);
            match length_type {
                // length type ID
                0 => {
                    let total = consume(data, 15);
                    let current = data.len();
                    println!("{} bits in sub packets", total);
                    while data.len() != (current - total) {
                        version += decode_packet(data);
                    }
                    println!("got all {} bits in sub packets", total);
                }
                1 => {
                    let mut length = consume(data, 11);
                    println!("{} sub packets", length);
                    while length > 0 {
                        version += decode_packet(data);
                        length -= 1;
                    }
                    println!("got all {} subpackets", length);
                }
                _ => {
                    println!("Unknown length type");
                    print_bitvec(data, 32);
                    unreachable!();
                }
            };
            //consume(data, length);
        } /*
          _ => {
              print_bitvec(data, 32);
              panic!(
                  "Unknown packet type {} (remaining: {})",
                  packet_type,
                  data.len()
              );
          }
          */
    }

    version
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
    // let versum = decode_versions2(&transmission);
    // assert_eq!(versum, 42);
}
