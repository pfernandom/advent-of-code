#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        collections::{BinaryHeap, HashSet},
        fs,
    };

    use crate::grid::Grid;

    type Coord = (i32, i32);

    struct HGrid(Grid<i32>);

    impl HGrid {
        fn print(&self) {
            self.0.print();
        }

        fn rows(&self) -> usize {
            self.0._grid.len()
        }

        fn cols(&self) -> usize {
            self.0._grid[0].len()
        }

        fn in_range(&self, row: i32, col: i32) -> bool {
            row >= 0 && row < self.rows() as i32 && col >= 0 && col < self.cols() as i32
        }

        fn print_mem(mem: &Vec<Vec<Vec<i32>>>, level: usize) {
            for r in mem {
                for c in r {
                    print!("{:>3}", c[level])
                }
                println!();
            }
        }
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d17.txt").expect("");
        let grid: HGrid = HGrid(Grid::new(
            contents
                .split("\n")
                .map(|l| {
                    l.split("")
                        .filter(|e| !e.is_empty())
                        .map(|c| {
                            i32::from_str_radix(format!("{}", c).as_str(), 10)
                                .expect(format!("can parse {}", c).as_str())
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ));

        let mut heap2: BinaryHeap<(i32, i32, i32, i32, i32, i32)> = BinaryHeap::new();

        heap2.push((0, 0, 0, 0, 0, 0));

        let mut visited2 = HashSet::new();

        while let Some((h, row, col, dr, dc, s)) = heap2.pop() {
            let h = -1 * h;
            if !grid.in_range(row, col) {
                continue;
            }

            if row == grid.rows() as i32 - 1 && col == grid.cols() as i32 - 1 {
                println!("res={}", h);
                break;
            }

            let key = (row, col, dr, dc, s);

            if visited2.contains(&key) {
                continue;
            }
            visited2.insert(key);

            if s < 3 && (dr, dc) != (0, 0) {
                let next_row = row as i32 + dr;
                let next_col = col as i32 + dc;
                if grid.in_range(next_row, next_col) {
                    heap2.push((
                        -1 * (h + grid.0._grid[next_row as usize][next_col as usize]),
                        next_row,
                        next_col,
                        dr,
                        dc,
                        s + 1,
                    ))
                }
            }

            for (next_dr, next_dc) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (next_dr, next_dc) != (dr, dc) && (next_dr, next_dc) != (-dr, -dc) {
                    let next_row = row + next_dr;
                    let next_col = col + next_dc;
                    if grid.in_range(next_row, next_col) {
                        heap2.push((
                            -1 * (h + grid.0._grid[next_row as usize][next_col as usize]),
                            next_row,
                            next_col,
                            next_dr,
                            next_dc,
                            1,
                        ))
                    }
                }
            }
        }

        println!("Done");
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d17.txt").expect("");
        let grid: HGrid = HGrid(Grid::new(
            contents
                .split("\n")
                .map(|l| {
                    l.split("")
                        .filter(|e| !e.is_empty())
                        .map(|c| {
                            i32::from_str_radix(format!("{}", c).as_str(), 10)
                                .expect(format!("can parse {}", c).as_str())
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ));

        let mut heap2: BinaryHeap<(i32, i32, i32, i32, i32, i32)> = BinaryHeap::new();

        heap2.push((0, 0, 0, 0, 0, 0));

        let mut visited2 = HashSet::new();

        while let Some((h, row, col, dr, dc, s)) = heap2.pop() {
            let h = -1 * h;
            if !grid.in_range(row, col) {
                continue;
            }

            if row == grid.rows() as i32 - 1 && col == grid.cols() as i32 - 1 {
                println!("res={}", h);
                break;
            }

            let key = (row, col, dr, dc, s);

            if visited2.contains(&key) {
                continue;
            }
            visited2.insert(key);

            if s < 10 && (dr, dc) != (0, 0) {
                let next_row = row as i32 + dr;
                let next_col = col as i32 + dc;
                if grid.in_range(next_row, next_col) {
                    heap2.push((
                        -1 * (h + grid.0._grid[next_row as usize][next_col as usize]),
                        next_row,
                        next_col,
                        dr,
                        dc,
                        s + 1,
                    ))
                }
            }

            if s > 3 || (dr, dc) == (0, 0) {
                for (next_dr, next_dc) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    if (next_dr, next_dc) != (dr, dc) && (next_dr, next_dc) != (-dr, -dc) {
                        let next_row = row + next_dr;
                        let next_col = col + next_dc;
                        if grid.in_range(next_row, next_col) {
                            heap2.push((
                                -1 * (h + grid.0._grid[next_row as usize][next_col as usize]),
                                next_row,
                                next_col,
                                next_dr,
                                next_dc,
                                1,
                            ))
                        }
                    }
                }
            }
        }

        println!("Done");
    }
}
