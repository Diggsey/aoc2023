const INPUT: &str = include_str!("../../inputs/day13.txt");

fn find_reflection(board: &[bool], rows: usize, cols: usize) -> Option<usize> {
    for x in 1..cols {
        if (0..x)
            .rev()
            .zip(x..cols)
            .all(|(x0, x1)| (0..rows).all(|y| board[y * cols + x0] == board[y * cols + x1]))
        {
            return Some(x);
        }
    }
    None
}

fn compute_reflection(board: &str) -> usize {
    let rows = board.lines().count();
    let board: Vec<_> = board
        .lines()
        .flat_map(|line| line.chars().map(|c| c == '#'))
        .collect();
    let cols = board.len() / rows;
    if let Some(pos) = find_reflection(&board, rows, cols) {
        pos
    } else {
        let board: Vec<_> = (0..cols)
            .flat_map(|y| {
                let board = &board;
                (0..rows).map(move |x| board[x * cols + y])
            })
            .collect();
        find_reflection(&board, cols, rows).unwrap() * 100
    }
}

fn main() {
    let sum: usize = INPUT.split("\n\n").map(compute_reflection).sum();

    println!("{sum}");
}
