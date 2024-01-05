#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use nalgebra::{RowVector3, Vector3};
    use std::{
        collections::{HashSet, VecDeque},
        fs,
    };

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d21.txt").expect("");
        let grid = Grid::new(contents.lines().map(|l| l.chars().collect()).collect());

        let start = grid.find(&'S').unwrap();

        println!("start:{:?}", start);

        let steps = 64;

        let answ = count_steps(steps, start, &grid);
        let mut grid1 = grid.clone();
        for (x, y) in &answ {
            grid1.set(*x, *y, 'X');
        }
        grid1.print();
        println!("{:?}", answ.len());
    }

    fn count_steps(steps: i32, start: (i32, i32), grid: &Grid<char>) -> HashSet<(i32, i32)> {
        let mut possible_steps: VecDeque<(i32, i32, i32)> =
            VecDeque::from([(start.0, start.1, steps)]);

        let mut found: HashSet<(i32, i32)> = HashSet::new();

        let mut answ = HashSet::new();

        while let Some((x, y, steps_to_go)) = possible_steps.pop_front() {
            if steps_to_go % 2 == 0 {
                answ.insert((x, y));
            }
            if steps_to_go == 0 {
                continue;
            }

            let directions = get_empty_positions(grid, x, y);

            for (col, row) in &directions {
                if found.contains(&(*col, *row)) {
                    continue;
                }
                found.insert((*col, *row));
                possible_steps.push_back((*col, *row, steps_to_go - 1));
            }
        }
        answ
    }

    #[test]
    fn p2() {
        let grid = get_expanded_grid((9, 9 as usize));

        let center = (grid.cols() / 2) as i32;
        assert_eq!(grid.get(center, center).unwrap(), &'S');

        let a = count_steps(65, (center, center), &grid).len();
        let b = count_steps(65 + 131, (center, center), &grid).len();
        let c = count_steps(65 + 131 * 2, (center, center), &grid).len();

        println!("a={}, b={}, c={}", a, b, c);

        let mb = Vector3::new(a as f64, b as f64, c as f64);
        let ma = nalgebra::Matrix3::from_rows(&[
            RowVector3::new(0.0, 0.0, 1.0),
            RowVector3::new(1.0, 1.0, 1.0),
            RowVector3::new(4.0, 2.0, 1.0),
        ]);
        let decomp = ma.lu();
        let x = decomp.solve(&mb).expect("Linear resolution failed.");
        println!("{:?}", x);

        let n = (26501365 / get_expanded_grid((1, 1 as usize)).cols()) as f64;
        let res = x[0] * n * n + x[1] * n + x[2];

        println!("res: {}", res);
    }

    fn mark(steps: i32, start: (i32, i32), grid1: &mut Grid<char>, grid: &Grid<char>) -> usize {
        // let mut grid1 = grid.clone();
        let answ = count_steps(steps, start, &grid);
        for (x, y) in &answ {
            if grid1.get(*x, *y).unwrap() == &'X' {
                grid1.set(*x, *y, 'O');
            } else {
                grid1.set(*x, *y, 'X');
            }
        }
        answ.len()
    }

    fn get_expanded_grid(expansions: (i32, usize)) -> Grid<char> {
        let contents = fs::read_to_string("./problems/d21.txt").expect("");

        let mut rows = Vec::new();

        for _ in 0..expansions.0 {
            for line in contents.lines() {
                let expanded_cols = vec![line; expansions.1].join("");
                rows.push(expanded_cols.chars().collect::<Vec<_>>())
            }
        }

        Grid::new(rows)
    }

    fn get_empty_positions(grid: &Grid<char>, x: i32, y: i32) -> VecDeque<(i32, i32)> {
        let directions: VecDeque<(i32, i32)> = grid
            .get_xy_directions_with_match(x, y, &HashSet::from(['.', 'S', 'O']))
            .iter()
            .map(|(a, b, _)| (*a, *b))
            .collect();
        directions
    }

    fn normalize_coords(grid: &Grid<char>, mut col: i32, mut row: i32) -> (i32, i32) {
        let rows = grid.rows();
        let cols = grid.cols();

        // println!("{},{}", col, row);

        if col < 0 {
            col = cols - (col.abs() % (cols));
        }

        if row < 0 {
            row = rows - (row.abs() % (rows));
        }

        if col >= cols {
            col = col % (cols);
        }

        if row >= rows {
            row = row % (rows);
        }
        (col, row)
    }

    fn get_with_repeat(grid: &Grid<char>, mut col: i32, mut row: i32) -> Option<&char> {
        (col, row) = normalize_coords(grid, col, row);
        let res = grid.get(col, row);

        assert!(res.is_some(), "element should be present: {}, {}", col, row);
        res
    }
}
