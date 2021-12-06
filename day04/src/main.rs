use itertools::Itertools;

type Board = Vec<Vec<(String, bool)>>;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut all_lines = input.lines().filter(|l| l != &"");

    let moves = all_lines.next().unwrap().split(",");

    let mut boards: Vec<Board> = vec![];
    for lines in all_lines.filter(|l| l != &"").chunks(5).into_iter() {
        let mut board = vec![];
        for line in lines {
            let row: Vec<(String, bool)> = line
                .split(' ')
                .filter(|p| p != &"")
                .map(|s| (s.to_owned(), false))
                .collect();
            board.push(row);
        }
        boards.push(board);
    }

    let mut winning_board: Board = vec![];
    let mut last_called_move: u32 = 0;
    for m in moves {
        for b in 0..boards.len() {
            for x in 0..boards[b].len() {
                for y in 0..boards[b][x].len() {
                    if m == boards[b][x][y].0 {
                        boards[b][x][y].1 = true;
                    }
                }
            }
            if board_wins(&boards[b].clone()) {
                winning_board = boards[b].clone();
                last_called_move = m.parse().unwrap();
                break;
            }
        }
        if winning_board.len() > 0 {
            break;
        }
    }

    let sum_of_unmarked_numbers: u32 = winning_board
        .into_iter()
        .flatten()
        .filter(|(_, marked)| !*marked)
        .map(|(x, _)| x.parse::<u32>().unwrap())
        .sum();

    println!("Part 1: {}", sum_of_unmarked_numbers * last_called_move);
}

fn board_wins(board: &Board) -> bool {
    for i in 0..5 {
        let row_match = board
            .clone()
            .into_iter()
            .nth(i)
            .unwrap()
            .into_iter()
            .all(|(_, marked)| marked);
        let col_match = board
            .clone()
            .into_iter()
            .map(|r| r[i].clone())
            .all(|(_, marked)| marked);
        if row_match || col_match {
            return true;
        }
    }
    false
}
