# N-queens solver

This repo contains a [N-queens problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle) solver which uses recursion and [backtracking](https://en.wikipedia.org/wiki/Backtracking) in its implementation.

Example solutions for the 4-queens problem:
![Solutions to the 4-queens problem](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)

## Usage
1. [Install Rust](https://www.rust-lang.org/tools/install).
1. Run example with `cargo run <n>`, where `<n>` is the board-size.

```bash
$ cargo run 4

2 solutions were found for the 4-queens problem:

Solution 1
".Q.."
"...Q"
"Q..."
"..Q."

Solution 2
"..Q."
"Q..."
"...Q"
".Q.."
```

```bash
$ cargo run 8

92 solutions were found for the 4-queens problem:

Solution 1
"Q......."
"....Q..."
".......Q"
".....Q.."
"..Q....."
"......Q."
".Q......"
"...Q...."

...
```

## Performance

Queens  | Solutions | Time
---     | ---       | --- 
4       | 2         | 0.1 ms
5       | 10        | 0.6 ms 
6       | 4         | 2.4 ms
7       | 40        | 10.0 ms
8       | 92        | 45.8 ms
9       | 352       | 227.2 ms
10      | 724       | 1.1 s
11      | 2680      | 6.1 s
12      | 14200     | 36.2 s
