#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

    use crate::grid::Grid;
    type VGrid = Vec<Vec<char>>;

    fn find_empty_rows_and_cols(grid: &VGrid) -> (HashSet<usize>, HashSet<usize>) {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut empty_rows: HashSet<usize> = HashSet::new();
        let mut empty_cols: HashSet<usize> = HashSet::new();

        for (r, row) in grid.iter().enumerate() {
            if row.iter().find(|c| **c != '.').is_none() {
                empty_rows.insert(r);
            }
        }

        let mut found = false;
        for c in 0..cols {
            for r in 0..rows {
                found = found
                    || match grid[r][c] {
                        '.' => false,
                        _ => true,
                    };
            }

            if !found {
                empty_cols.insert(c);
                // break;
            }
            found = false;
        }
        (empty_rows, empty_cols)
    }

    fn expand(grid: VGrid, scale: usize) -> VGrid {
        let (empty_rows, empty_cols) = find_empty_rows_and_cols(&grid);
        let g = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                if empty_rows.contains(&r) {
                    let mut new_rows = Vec::new();
                    for _ in 0..scale {
                        new_rows.push(row.clone());
                    }
                    new_rows
                } else {
                    vec![row.clone()]
                }
            })
            .map(|row| {
                row.iter()
                    .enumerate()
                    .flat_map(|(c, col)| {
                        if empty_cols.contains(&c) {
                            let mut new_cols = Vec::new();
                            for _ in 0..scale {
                                new_cols.push(*col);
                            }
                            new_cols
                        } else {
                            vec![*col]
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        g
    }

    fn print(grid: &VGrid) {
        for row in grid {
            let s = row.iter().collect::<String>();
            println!("{}", s);
        }
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d11.txt").expect("");
        let mut g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        g = expand(g, 2);

        // print(&g);

        let grid = Grid::new(g);

        let stars = grid.find_all(&'#');
        println!("stars={:?}", stars);

        let mut star_paths = HashSet::new();
        let mut dists = Vec::new();
        let mut sum_dist = 0;
        for s1 in &stars {
            for s2 in &stars {
                star_paths.insert((*s1, *s2));

                let dist = i32::abs(s1.0 - s2.0) + i32::abs(s1.1 - s2.1);
                sum_dist += dist;

                dists.push(dist)
            }
        }

        println!("sum_dist:{:?}", sum_dist / 2);

        assert!((sum_dist / 2 == 9370588) || (sum_dist / 2 == 374))
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d11.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // g = expand(g, 1000000);
        let (empty_rows, empty_cols) = find_empty_rows_and_cols(&g);

        println!("empty_rows={:?}, empty_cols={:?}", empty_rows, empty_cols);
        // print(&g);

        // let mut empty_rows = empty_rows.iter().collect::<Vec<_>>();
        // let mut empty_cols = empty_cols.iter().collect::<Vec<_>>();

        // empty_rows.sort();
        // empty_cols.sort();

        let mut rinc = 0;

        let mut incs: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let scale = 1000000;
        for (r, row) in g.iter().enumerate() {
            let mut cinc = 0;
            if empty_rows.contains(&r) {
                rinc += scale - 1;
            }
            for (c, _) in row.iter().enumerate() {
                if empty_cols.contains(&c) {
                    cinc += scale - 1;
                }

                incs.insert((r as i32, c as i32), (rinc, cinc));
            }
        }

        // println!("incs={:?}", incs);

        let grid = Grid::new(g);

        let stars = grid.find_all(&'#');

        // for (i, s) in stars.iter().enumerate() {
        //     grid.set(
        //         s.0,
        //         s.1,
        //         char::from_digit((i + 1) as u32, 10).expect(format!("start={}", i + 1).as_str()),
        //     );
        // }

        grid.print();

        let stars = stars
            .iter()
            .map(|start: &(i32, i32)| {
                let (inc_r, inc_c) = incs.get(&(start.1, start.0)).unwrap();
                // println!(
                //     "start={:?}, inc_r={}, inc_c={}",
                //     grid.get(start.0, start.1),
                //     inc_r,
                //     inc_c
                // );
                println!(
                    "Star location: ({:?}) {:?}",
                    grid.get(start.0, start.1),
                    (inc_c + start.0, inc_r + start.1)
                );
                (inc_c + start.0 - 1, inc_r + start.1 - 1)
            })
            .collect::<HashSet<_>>();
        // println!("stars={:?}", stars);

        let mut star_paths = HashSet::new();
        let mut dists = Vec::new();
        let mut sum_dist: i64 = 0;
        for s1 in &stars {
            for s2 in &stars {
                star_paths.insert((*s1, *s2));

                let dist: i64 = (i32::abs(s1.0 - s2.0) + i32::abs(s1.1 - s2.1)).into();
                sum_dist += dist;

                dists.push(dist)
            }
        }

        println!("sum_dist:{:?}", sum_dist / 2);
    }
}
