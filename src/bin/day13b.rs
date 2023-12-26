const INPUT: &str = include_str!("../../inputs/day13.txt");

fn find_reflection(board: &[bool], rows: usize, cols: usize, ignore: usize) -> Option<usize> {
    for x in 1..cols {
        if x != ignore {
            if (0..x)
                .rev()
                .zip(x..cols)
                .all(|(x0, x1)| (0..rows).all(|y| board[y * cols + x0] == board[y * cols + x1]))
            {
                return Some(x);
            }
        }
    }
    None
}

fn compute_reflection(
    board: &[bool],
    rows: usize,
    cols: usize,
    ignore_x: usize,
    ignore_y: usize,
) -> Option<usize> {
    Some(
        if let Some(pos) = find_reflection(board, rows, cols, ignore_x) {
            pos
        } else {
            let board: Vec<_> = (0..cols)
                .flat_map(|y| {
                    let board = &board;
                    (0..rows).map(move |x| board[x * cols + y])
                })
                .collect();
            find_reflection(&board, cols, rows, ignore_y)? * 100
        },
    )
}

fn compute_unsmudged_reflection(board: &str) -> usize {
    let rows = board.lines().count();
    let mut board: Vec<_> = board
        .lines()
        .flat_map(|line| line.chars().map(|c| c == '#'))
        .collect();
    let cols = board.len() / rows;

    let smudged_reflection = compute_reflection(&board, rows, cols, 0, 0).unwrap();
    let ignore_x = smudged_reflection % 100;
    let ignore_y = smudged_reflection / 100;

    for i in 0..board.len() {
        board[i] = !board[i];
        if let Some(unsmudged_reflection) =
            compute_reflection(&board, rows, cols, ignore_x, ignore_y)
        {
            return unsmudged_reflection;
        }
        board[i] = !board[i];
    }
    unreachable!()
}

fn main() {
    let sum: usize = INPUT.split("\n\n").map(compute_unsmudged_reflection).sum();

    println!("{sum}");
}
