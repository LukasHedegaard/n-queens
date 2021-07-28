use std::env;
use std::time::Instant;

// We'll take a recuesive approach with backtracking
struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut sols = Vec::new();
        let mut board = vec![vec!['_'; n]; n];

        Solution::run(&mut board, &mut sols, 0, 0);

        return sols;
    }

    fn run(
        board: &mut Vec<Vec<char>>,
        sols: &mut Vec<Vec<String>>,
        start_row: usize,
        start_col: usize,
    ) {
        for row in start_row..board.len() {
            for col in start_col..board.len() {
                if board[row][col] == '_' {
                    for &val in ['Q', '.'].iter() {
                        if Solution::valid(val, board, row, col) {
                            // Try
                            board[row][col] = val;
                            Solution::run(board, sols, row, 0);

                            // Backtrack
                            board[row][col] = '_';
                        }
                    }
                    return;
                }
            }
        }
        let mut sol = Vec::with_capacity(board.len());
        for row in 0..board.len() {
            sol.push(board[row].iter().collect());
        }
        sols.push(sol);
    }

    fn valid(val: char, board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        if val == 'Q' {
            return Solution::valid_queen(board, row, col);
        } else {
            return Solution::valid_space(board, row, col);
        }
    }

    fn valid_queen(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        // Check row
        for &c in &board[row] {
            if c == 'Q' {
                return false;
            }
        }

        // Check col
        for r in board {
            if r[col] == 'Q' {
                return false;
            }
        }

        // Check diags
        let mut j;
        for (i, r) in board.iter().enumerate() {
            j = (i as i32 - row as i32).abs() as usize;
            if i != row {
                if col + j < board.len() && r[col + j] == 'Q' {
                    return false;
                }
                if col >= j && r[col - j] == 'Q' {
                    return false;
                }
            }
        }

        return true;
    }

    fn valid_space(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        let mut x = 0;
        // Check row
        for (i, &c) in board[row].iter().enumerate() {
            if i != col && (c == 'Q' || c == '_') {
                x += 1;
                break;
            }
        }

        // Check col
        for (i, r) in board.iter().enumerate() {
            if i != row && (r[col] == 'Q' || r[col] == '_') {
                x += 1;
                break;
            }
        }

        return x == 2;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 2,
        "Only a single argument should be given, which contains the board-size."
    );
    let n: i32 = args[1].parse::<i32>().unwrap();

    let now = Instant::now();
    let sols: Vec<Vec<String>> = Solution::solve_n_queens(n);

    println!(
        "{} solutions were found for the {}-queens problem in {} ms:",
        sols.len(),
        n,
        now.elapsed().as_millis()
    );

    for (i, sol) in sols.iter().enumerate() {
        println!("\nSolution {}", i + 1);
        for row in sol.iter() {
            println!("{:?}", row);
        }
    }
}
