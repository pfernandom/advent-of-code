#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use crate::grid::Grid;

    enum Dir {
        L,
        D,
        R,
        U,
    }

    impl Dir {
        fn to_direction(&self) -> (i32, i32) {
            match self {
                Dir::L => (0, -1),
                Dir::D => (1, 0),
                Dir::R => (0, 1),
                Dir::U => (-1, 0),
            }
        }
    }

    fn solve(data: Vec<(Dir, i32, &str)>) {
        let mut row = 0;
        let mut col = 0;

        let mut max_row = i32::MIN;
        let mut max_col = i32::MIN;
        let mut min_row = i32::MAX;
        let mut min_col = i32::MAX;

        for (dir, dist, _) in &data {
            (row, col) = match dir {
                Dir::D => (row + dist, col),
                Dir::U => (row - dist, col),
                Dir::L => (row, col - dist),
                Dir::R => (row, col + dist),
            };
            max_row = max_row.max(row);
            max_col = max_col.max(col);
            min_row = min_row.min(row);
            min_col = min_col.min(col);
        }

        let rows = min_row.abs_diff(max_row + 1);
        let cols = min_col.abs_diff(max_col + 1);
        let offset_row = min_row.abs_diff(0);
        let offset_col = min_col.abs_diff(0);

        println!("{},{}", rows, cols);
        println!("offsets: {},{}", offset_row, offset_col);

        let mut grid = vec![vec!['.'; cols as usize]; rows as usize];

        let mut row = offset_row as i32;
        let mut col = offset_col as i32;
        for (dir, dist, _) in &data {
            let (nrow, ncol) = match dir {
                Dir::D => (row + dist, col),
                Dir::U => (row - dist, col),
                Dir::L => (row, col - dist),
                Dir::R => (row, col + dist),
            };

            let nrow = nrow as usize;
            let ncol = ncol as usize;
            let crow = row as usize;
            let ccol = col as usize;

            // let mrow = crow.min(nrow);

            // println!("{:?}", crow..nrow);

            if crow < nrow {
                for i in crow..nrow + 1 {
                    grid[i][ccol] = '#';
                }
            }

            if crow > nrow {
                for i in nrow..crow + 1 {
                    grid[i][ccol] = '#';
                }
            }

            if ccol < ncol {
                for i in ccol..ncol + 1 {
                    grid[crow][i] = '#';
                }
            }

            if ccol > ncol {
                for i in ncol..ccol + 1 {
                    grid[crow][i] = '#';
                }
            }

            row = nrow as i32;
            col = ncol as i32;
        }

        // for row in &grid {
        //     println!("{}", row.iter().collect::<String>());
        // }

        let mut g = Grid::new(grid);
        fill(&mut g);

        // g.print();

        for (col, row) in g.find_all(&'.') {
            g.set(col, row, '#');
        }

        // g.print();

        println!("res={}", g.find_all(&'#').len() as i32);
    }

    fn solve2(data: Vec<(Dir, i32, &str)>) {
        let mut row = 0;
        let mut col = 0;

        let mut max_row = i32::MIN;
        let mut max_col = i32::MIN;
        let mut min_row = i32::MAX;
        let mut min_col = i32::MAX;

        let mut vertices = Vec::new();
        let mut boundaries: f64 = 0.0;
        for (dir, dist, _) in &data {
            let (nrow, ncol) = match dir {
                Dir::D => (row + dist, col),
                Dir::U => (row - dist, col),
                Dir::L => (row, col - dist),
                Dir::R => (row, col + dist),
            };

            boundaries += *dist as f64;

            max_row = max_row.max(nrow);
            max_col = max_col.max(ncol);
            min_row = min_row.min(nrow);
            min_col = min_col.min(ncol);

            vertices.push((nrow as f64, ncol as f64));

            row = nrow as i32;
            col = ncol as i32;
        }

        let area = shoelace_formula(&vertices);
        println!("Inner area:{:?}", area);
        println!("Perimeter: {:?}", boundaries);
        println!("Area: {:?}", picks_formula(boundaries, area));
    }

    fn picks_formula(bounderies: f64, area: f64) -> f64 {
        let i = area - bounderies / 2.0 + 1.0;
        i + bounderies
    }

    fn shoelace_formula(coordinates: &Vec<(f64, f64)>) -> f64 {
        let mut area: f64 = 0.0;
        let n = coordinates.len();

        // Sum over the main diagonal
        for i in 0..n - 1 {
            area += coordinates[i].0 * coordinates[i + 1].1;
        }
        area += coordinates[n - 1].0 * coordinates[0].1; // Close the polygon

        // Subtract the other diagonal
        for i in 0..n - 1 {
            area -= coordinates[i + 1].0 * coordinates[i].1;
        }
        area -= coordinates[0].0 * coordinates[n - 1].1; // Close the polygon

        // Taking absolute value and dividing by 2
        area.abs() / 2.0
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d18.txt").expect("");

        let data: Vec<(Dir, i32, &str)> = contents
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|fields| {
                (
                    match fields[0] {
                        "R" => Dir::R,
                        "D" => Dir::D,
                        "L" => Dir::L,
                        "U" => Dir::U,
                        _ => unreachable!(),
                    },
                    i32::from_str_radix(fields[1], 10).unwrap(),
                    fields[2],
                )
            })
            .collect::<Vec<_>>();

        solve2(data);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d18.txt").expect("");

        let data = contents
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|fields| fields[2].replace("(#", "").replace("(", ""))
            .map(|fields| {
                println!(
                    "{},{}",
                    &fields[5..6],
                    i32::from_str_radix(&fields[0..5], 16).unwrap()
                );
                (
                    match &fields[5..6] {
                        "0" => Dir::R,
                        "1" => Dir::D,
                        "2" => Dir::L,
                        "3" => Dir::U,
                        _ => unreachable!(),
                    },
                    i32::from_str_radix(&fields[0..5], 16).unwrap(),
                    "   ",
                )
            })
            .collect::<Vec<_>>();

        solve2(data);
    }

    fn fill(grid: &mut Grid<char>) {
        for row in 0..grid.rows() {
            grid.flood(0, row, HashSet::from(['.']), 'P');
            grid.flood(grid.cols() - 1, row, HashSet::from(['.']), 'P');
        }
        for col in 0..grid.cols() {
            grid.flood(col, 0, HashSet::from(['.']), 'P');
            grid.flood(col, grid.rows() - 1, HashSet::from(['.']), 'P');
        }
    }
}
