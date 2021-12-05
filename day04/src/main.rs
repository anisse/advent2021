type Board = Vec<Vec<u8>>;
type BoardRef<'a> = &'a [Vec<u8>];

fn main() {
    let (boards, draws) = parse(include_str!("../input.txt"));
    //part 1
    let win_score = winning_score(&boards, &draws);
    println!("Winning board score: {}", win_score);
    //part 2
    let lose_score = losing_score(&boards, &draws);
    println!("Losing board score: {}", lose_score);
}
fn parse(input: &str) -> (Vec<Board>, Vec<u8>) {
    let mut lines = input.lines();

    let draws = lines
        .next()
        .expect("No draw line")
        .split(',')
        .map(|x| x.parse::<u8>().expect("not int"))
        .collect();

    let mut boards = Vec::new();

    while let Some(space) = lines.next() {
        assert_eq!(space, ""); // skip space lines
        let board = lines
            .by_ref()
            .take(5)
            .map(|row| {
                row.split_ascii_whitespace()
                    .map(|x| x.parse::<u8>().expect("not int"))
                    .collect()
            })
            .collect();
        boards.push(board);
    }

    (boards, draws)
}

fn winning_score(boards: &[Board], draws: &[u8]) -> usize {
    let mut drawn = vec![vec![vec![false; 5]; 5]; boards.len()];
    for draw in draws.iter() {
        for (i, board) in boards.iter().enumerate() {
            if let Some((row, col)) = board_has(board, *draw) {
                drawn[i][row][col] = true;
                if is_winning(&drawn[i], row, col) {
                    return score_sum(board, &drawn[i]) * board[row][col] as usize;
                }
            }
        }
    }
    0
}

fn board_has(board: BoardRef, num: u8) -> Option<(usize, usize)> {
    for (row, l) in board.iter().enumerate() {
        for (col, v) in l.iter().enumerate() {
            if *v == num {
                return Some((row, col));
            }
        }
    }
    None
}

fn is_winning(drawn: &[Vec<bool>], row: usize, col: usize) -> bool {
    // check row
    drawn[row].iter().all(|i| *i) ||
        //check col
        drawn.iter().all(|i| i[col])
}
fn score_sum(board: BoardRef, drawn: &[Vec<bool>]) -> usize {
    board
        .iter()
        .zip(drawn.iter())
        .map(|(row, d_row)| {
            row.iter()
                .zip(d_row.iter())
                .filter(|(_, d)| !**d)
                .map(|(x, _)| *x as usize)
                .sum::<usize>()
        })
        .sum()
}

fn losing_score(boards: &[Board], draws: &[u8]) -> usize {
    let mut drawn = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut won = 0;
    let mut winners = vec![false; boards.len()];
    for draw in draws.iter() {
        for (i, board) in boards.iter().enumerate() {
            if winners[i] {
                continue;
            }
            if let Some((row, col)) = board_has(board, *draw) {
                drawn[i][row][col] = true;
                if is_winning(&drawn[i], row, col) {
                    won += 1;
                    winners[i] = true;
                    if won == boards.len() {
                        // this is the last one
                        return score_sum(board, &drawn[i]) * board[row][col] as usize;
                    }
                }
            }
        }
    }
    0
}

#[test]
fn test() {
    let (boards, draws) = parse(include_str!("../sample.txt"));
    //part 1
    let win_score = winning_score(&boards, &draws);
    assert_eq!(win_score, 4512);
    //part 2
    let lose_score = losing_score(&boards, &draws);
    assert_eq!(lose_score, 1924);
}
