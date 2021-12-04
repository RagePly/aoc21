// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day4");

#[cfg(not(feature = "day4"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(4, 1)
}

#[cfg(not(feature = "day4"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(4, 2)
}

// v START SOLUTION v

#[cfg(feature = "day4")]
type Board = Vec<Vec<(i32, bool)>>;


#[cfg(feature = "day4")]
fn play(board: &mut Board, number: i32) -> bool {
    // check inclusion
    let mut row:    usize   = 0;
    let mut column: usize   = 0;
    let mut found:  bool    = false;
    for (y, r) in board.iter().enumerate() {
        for (x, (n, _)) in r.iter().enumerate() {
            if *n == number {
                row = y;
                column = x;
                found = true;
                break;
            }
        }
        if found {
            break
        }
    }

    if !found {
        false
    } else {
        // set state
        board[row][column].1 = true;

        // check row
        let mut bingo: bool = true;
        for y in 0..board.len() {
            if !board[y][column].1 {
                bingo = false;
                break;
            }
        }

        if bingo {
            return true
        }


        // check column
        bingo = true;
        for x in 0..board[0].len() {
            if !board[row][x].1 {
                bingo = false;
                break;
            }
        }

        if bingo {
            return true
        }

        false
    }
}

#[cfg(feature = "day4")]
fn sum_unmarked(board: &Board) -> i32 {
    board.iter().map(
        |row| row.iter().filter(|b| !b.1).map(|b| b.0).sum::<i32>()
    ).sum()
}

#[cfg(feature = "day4")]
pub fn part1(source: String) -> i32 {
    let mut lines = source.split("\r\n");
    let order = lines.next().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut boards: Vec<Board> = Vec::new();
    let mut board_buffer: Board = Board::new();
    loop {
        match lines.next() {
            Some("") => {
                if board_buffer.len() > 0 {
                    boards.push(board_buffer.clone());
                    board_buffer.clear();
                }
            }
            Some(line) => {
                board_buffer.push(line.split_ascii_whitespace().map(|n| (n.parse::<i32>().unwrap(), false)).collect())
            }
            None => {
                boards.push(board_buffer.clone());
                break
            }
        }
    }
    for number in order {
        for board in &mut boards {
            if play(board, number) {
                return number*sum_unmarked(board)
            }
        }
    }
    unreachable!()
}

#[cfg(feature = "day4")]
pub fn part2(source: String) -> i32 {
    let mut lines = source.split("\r\n");
    let order = lines.next().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut boards: Vec<Board> = Vec::new();
    let mut board_buffer: Board = Board::new();
    loop {
        match lines.next() {
            Some("") => {
                if board_buffer.len() > 0 {
                    boards.push(board_buffer.clone());
                    board_buffer.clear();
                }
            }
            Some(line) => {
                board_buffer.push(line.split_ascii_whitespace().map(|n| (n.parse::<i32>().unwrap(), false)).collect())
            }
            None => {
                boards.push(board_buffer.clone());
                break
            }
        }
    }

    let worst_play: (usize, i32, &mut Board) = boards.iter_mut().map(
        |board| {
            for (i, n) in order.iter().enumerate() {
                if play(board, *n) {
                    return (i + 1, *n, board)
                }
            }
            unreachable!()
        }
    ).max_by(|(rounds1, _, _), (rounds2, _, _)| rounds1.cmp(rounds2)).unwrap();

    worst_play.1 * sum_unmarked(worst_play.2)
}
