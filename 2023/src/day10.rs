#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        fs::{self, File},
        io::{LineWriter, Write},
        iter::zip,
    };

    use crate::{assertions::assert_contains_all, grid::Grid};

    #[derive(Clone)]
    struct TubeGrid {
        grid: Grid<char>,
    }

    impl TubeGrid {
        fn new(grid: Grid<char>) -> Self {
            Self { grid }
        }

        fn find(&self, val: &char) -> Option<(i32, i32)> {
            self.grid.find(val)
        }

        fn xy_directions(&self, x: i32, y: i32) -> Vec<(i32, i32, &char)> {
            self.grid.xy_directions(x, y)
        }

        fn set(&mut self, x: i32, y: i32, val: char) -> bool {
            self.grid.set(x, y, val)
        }

        fn get(&self, x: i32, y: i32) -> Option<&char> {
            self.grid.get(x, y)
        }

        fn print(&self) {
            self.grid.print()
        }

        fn save_to_file(&self, path: &str) {
            self.grid.save_to_file(path);
        }

        fn get_connections(&self, x: i32, y: i32) -> Option<Vec<(i32, i32, char)>> {
            let dirs = match self.grid.get(x, y) {
                Some(c) => match c {
                    '|' => Some(vec![(x, y - 1), (x, y + 1)]),
                    'L' => Some(vec![(x, y - 1), (x + 1, y)]),
                    'F' => Some(vec![(x, y + 1), (x + 1, y)]),
                    'J' => Some(vec![(x, y - 1), (x - 1, y)]),
                    '7' => Some(vec![(x, y + 1), (x - 1, y)]),
                    '-' => Some(vec![(x + 1, y), (x - 1, y)]),
                    'S' => {
                        let dirs = self.grid.xy_directions(x, y);
                        Some(
                            dirs.iter()
                                .filter(|(nx, ny, _)| {
                                    let concon = self.get_connections(*nx, *ny);
                                    let points_to_s = match concon {
                                        None => false,
                                        Some(l) => l.iter().find(|nn| nn.2 == 'S').is_some(),
                                    };
                                    // println!("{:?}, points_to_s={}", concon, points_to_s);
                                    points_to_s
                                })
                                .map(|(x, y, _)| (*x, *y))
                                .collect::<Vec<_>>(),
                        )
                    }
                    _ => None,
                },
                None => None,
            };

            dirs.map(|d| {
                d.iter()
                    .filter_map(|(x, y)| self.grid.get_with_coordinates(*x, *y))
                    .map(|(x, y, c)| (x, y, *c))
                    .collect::<Vec<_>>()
            })
        }

        fn get_xy_directions_with_match(
            &self,
            x: i32,
            y: i32,
            cs: HashSet<char>,
        ) -> Vec<(i32, i32, &char)> {
            let next: Vec<(i32, i32, &char)> = self.grid.xy_directions(x, y);
            next.iter()
                .filter_map(|d| if cs.contains(d.2) { Some(*d) } else { None })
                .collect::<Vec<_>>()
        }

        fn clear_nonx(&mut self) {
            self.grid._grid = self
                .grid
                ._grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| match *c {
                            'X' => 'X',
                            'S' => 'X',
                            '.' => '.',
                            _ => '+',
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        fn flood(&mut self, x: i32, y: i32, cs: HashSet<char>) -> bool {
            if let Some(c) = self.get(x, y) {
                if !cs.contains(c) {
                    return false;
                }
            }

            let first = self
                .get_xy_directions_with_match(x, y, cs.clone())
                .iter()
                .map(|(x, y, c)| (*x, *y, **c))
                .collect::<Vec<_>>();

            if first.is_empty() {
                return false;
            }

            println!("first={:?}", first);

            let mut next = first.clone();
            while !next.is_empty() {
                let (x, y, _) = next.pop().unwrap();
                self.set(x, y, 'P');

                for n in self
                    .get_xy_directions_with_match(x, y, cs.clone())
                    .iter()
                    .map(|(x, y, c)| (*x, *y, **c))
                    .collect::<Vec<_>>()
                {
                    next.push(n);
                }
            }
            return true;
        }
    }

    #[test]
    fn are_connected_test() {
        let contents = fs::read_to_string("./problems/d10_sample.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let grid = TubeGrid::new(Grid::new(g));

        println!("{:?}", grid.get_connections(0, 4));

        // L
        assert_contains_all(
            grid.get_connections(0, 4).unwrap(),
            vec![(0, 3, '|'), (1, 4, 'J')],
        );

        // |
        assert_contains_all(
            grid.get_connections(3, 1).unwrap(),
            vec![(3, 0, '7'), (3, 2, 'L')],
        );

        // 7
        assert_contains_all(
            grid.get_connections(3, 0).unwrap(),
            vec![(2, 0, 'F'), (3, 1, '|')],
        );

        assert_contains_all(
            grid.get_connections(2, 0).unwrap(),
            vec![(3, 0, '7'), (2, 1, 'J')],
        )
    }

    fn grid_walk(g: TubeGrid) -> TubeGrid {
        let mut grid = g.clone();
        let start = grid.find(&'S').unwrap();

        println!("start={:?}", start);

        let mut pos = start.clone();
        let mut visited = HashSet::new();
        visited.insert((pos.0, pos.1, 'S'));

        let mut distances1: Vec<(i32, i32, i32)> = Vec::new();
        let mut distances2: Vec<(i32, i32, i32)> = Vec::new();

        let mut i = 0;
        let mut j = 10000;
        let mut found_start = false;
        while let Some(next) = grid.get_connections(pos.0, pos.1) {
            let has_start = next.iter().find(|(_, _, c)| *c == 'S').is_some();
            if has_start && i > 1 {
                grid.set(pos.0, pos.1, 'X');
                found_start = true;
                println!("End: pos={:?}, start={:?}", pos, start);
                break;
            }

            match next
                .iter()
                .find(|(x, y, c)| !visited.contains(&(*x, *y, *c)) && *c != 'X')
            {
                Some((x, y, c)) => {
                    // println!("{:?}", (x, y, c));
                    visited.insert((*x, *y, *c));
                    distances1.push((*x, *y, i));
                    distances2.push((*x, *y, j));
                    if grid.get(pos.0, pos.1).unwrap() != &'S' {
                        // grid.set(pos.0, pos.1, '!');
                        // grid.print();
                        grid.set(pos.0, pos.1, 'X');
                    }

                    // grid.print();
                    pos = (*x, *y);
                }
                None => {
                    println!("{:?}", grid.get(pos.0, pos.1));
                    grid.set(pos.0, pos.1, '!');
                    grid.print();
                    grid.set(pos.0, pos.1, 'X');
                    break;
                }
            }

            i += 1;
            j -= 1;

            // if i > 10000 {
            //     println!("timeout");
            //     break;
            // }
        }
        assert!(found_start);
        grid
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d10.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut grid = TubeGrid::new(Grid::new(g));
        let start = grid.find(&'S').unwrap();

        println!("start={:?}", start);

        let mut pos = start.clone();
        let mut visited = HashSet::new();
        visited.insert((pos.0, pos.1, 'S'));

        let mut distances1: Vec<(i32, i32, i32)> = Vec::new();
        let mut distances2: Vec<(i32, i32, i32)> = Vec::new();

        let mut i = 0;
        let mut j = 10000;
        while let Some(next) = grid.get_connections(pos.0, pos.1) {
            let has_start = next.iter().find(|(_, _, c)| *c == 'S').is_some();
            if has_start && pos != start {
                println!("End: pos={:?}, start={:?}", pos, start);
                break;
            }

            match next
                .iter()
                .find(|(x, y, c)| !visited.contains(&(*x, *y, *c)) && *c != 'X')
            {
                Some((x, y, c)) => {
                    // println!("{:?}", (x, y, c));
                    visited.insert((*x, *y, *c));
                    distances1.push((*x, *y, i));
                    distances2.push((*x, *y, j));
                    grid.set(pos.0, pos.1, 'X');
                    // grid.print();
                    pos = (*x, *y);
                }
                None => break,
            }

            i += 1;
            j -= 1;

            // if i > 10000 {
            //     println!("timeout");
            //     break;
            // }
        }

        println!("{:?}", grid.get(pos.0, pos.1));
        println!("{:?}", grid.get_connections(pos.0, pos.1));

        let min = distances2
            .iter()
            .fold(1000000000, |a, b| if a < b.2 { a } else { b.2 });

        distances2 = distances2
            .iter()
            .map(|(x, y, c)| (*x, *y, c - min))
            .collect::<Vec<_>>();

        let distances3 = zip(&distances1, &distances2)
            .map(|(d1, d2)| if d1.2 < d2.2 { d1 } else { d2 })
            .collect::<Vec<_>>();
        let middle = zip(&distances1, &distances2).find(|(a, b)| a.2 == b.2);

        // grid.save_to_file("./out_map.txt");

        let g2: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let trans = g2
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| {
                        let found = distances3
                            .iter()
                            .find(|(x, y, _)| (*y == (i as i32) && *x == (j as i32)) || *c == 'S');
                        if found.is_some() {
                            // println!("{}", c);
                            (format!("{}", *c), format!("{:^7}", found.unwrap().2))
                        } else {
                            (".".to_string(), format!("{:^7}", "."))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let file = File::create("out_map.txt").expect("file");
        let mut file = LineWriter::new(file);

        let file2 = File::create("out_map_2.txt").expect("file");
        let mut file2 = LineWriter::new(file2);

        for t in trans {
            // println!("{:?}", t);
            let s1 = t.iter().map(|e| e.0.clone()).collect::<String>();
            let s2 = t.iter().map(|e| e.1.clone()).collect::<String>();
            let d1 = s1.replace("'", "").replace(",", "").replace(" ", "");
            let d2 = s2;
            file.write(format!("{}\n", d1).as_bytes()).unwrap();
            file2.write(format!("{}\n", d2).as_bytes()).unwrap();
        }

        grid.print();

        // let mut mmax = 0;
        // for el in distances3 {
        //     mmax = mmax.max(el.2);
        // }

        let result = middle.unwrap().0 .2 + 1;
        // println!("d1:{:?}", distances1);
        // println!("d2:{:?}", distances2);
        println!("result: {:?}", result);
        // println!("{:?}", next)

        // for (x, y, c) in grid.xy_directions(start.0, start.1) {}
    }

    fn flood_grid(g: TubeGrid, flood_chars: HashSet<char>) -> TubeGrid {
        let mut grid = g.clone();
        let width = &grid.grid._grid.len();
        let height = &grid.grid._grid[0].len();

        let chars = flood_chars.clone();

        for y in 0..*width {
            if grid.flood(0, y as i32, chars.clone()) {
                println!("{}, {}", 0, y);
                grid.print();
            }

            if grid.flood((*height - 1) as i32, y as i32, chars.clone()) {
                println!("{}, {}", height - 1, y);
                grid.print();
            }
        }

        for x in 0..*height {
            if grid.flood(x as i32, 0, chars.clone()) {
                println!("{}, {}", x, 0);
                grid.print();
            }

            if grid.flood(x as i32, (*width - 1) as i32, chars.clone()) {
                println!("{}, {}", x, (*width - 1));
                grid.print();
            }
        }
        grid
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d10.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut grid = TubeGrid::new(Grid::new(g.clone()));
        grid.print();

        grid = grid_walk(grid);

        println!("Walked:");
        grid.print();

        grid = flood_grid(
            grid,
            HashSet::from(['.', '+', '|', '-', 'L', 'J', '7', 'F']),
        );

        println!("Flooded:");
        grid.print();

        let g2 = g
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        grid.get(x as i32, y as i32).map(|c2| {
                            if *c2 == 'P' {
                                '.'
                            } else if *c2 == 'S' {
                                'S'
                            } else if *c2 != 'X' {
                                '^'
                            } else {
                                *c
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        println!("Cleared after flood:");
        Grid::new(g2.clone()).print();

        // for (y, row) in g.iter().enumerate() {
        //     for (x, c) in row.iter().enumerate() {

        //     }
        // }

        let gwide = widen_xy(g2);
        grid = TubeGrid::new(Grid::new(gwide));

        println!("Post widen:");
        grid.print();

        let mut file = LineWriter::new(File::create("pre_flood.txt").expect("file"));

        for t in &grid.grid._grid {
            let s = t
                .iter()
                .map(|c| match *c {
                    'X' => 'X',
                    'S' => 'X',
                    '.' => '.',
                    _ => ' ',
                })
                .collect::<String>()
                .replace("P", " ")
                .replace("+", " ");
            // println!("{:?}", t);

            file.write(format!("{}\n", s).as_bytes()).unwrap();
        }

        grid = flood_grid(grid, HashSet::from(['.', '+', '^']));

        grid.print();

        let post = HashSet::from(['.', '^']);

        let mut tiles = 0;
        for row in &grid.grid._grid {
            for c in row {
                if post.contains(c) {
                    tiles += 1;
                }
            }
        }

        let mut file = LineWriter::new(File::create("out_map.txt").expect("file"));
        let mut file2 = LineWriter::new(File::create("out_map_2.txt").expect("file"));

        for t in grid.grid._grid {
            let s = t
                .iter()
                .map(|c| match *c {
                    'X' => 'X',
                    '.' => '.',
                    _ => ' ',
                })
                .collect::<String>()
                .replace("P", " ")
                .replace("+", " ");
            // println!("{:?}", t);

            file.write(format!("{}\n", s).as_bytes()).unwrap();
            file2
                .write(format!("{}\n", s.replace(" ", "")).as_bytes())
                .unwrap();
        }

        println!("result: {:?}", tiles);
        // println!("{:?}", next)

        // for (x, y, c) in grid.xy_directions(start.0, start.1) {}
    }

    fn widen(g: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut ngrid: Vec<Vec<char>> = Vec::new();

        for el in g {
            let mut nrow: Vec<char> = Vec::new();

            let mut last = '_';
            for w2 in el.windows(2) {
                match w2 {
                    &['-', b] => {
                        nrow.push('-');
                        nrow.push('-');
                        nrow.push(b);
                    }
                    &[a, '-'] => {
                        nrow.push(a);
                        nrow.push('-');
                        nrow.push('-');
                    }
                    &['F', '7'] => {
                        nrow.push('F');
                        nrow.push('-');
                        nrow.push('7');
                    }
                    &['F', 'J'] => {
                        nrow.push('F');
                        nrow.push('-');
                        nrow.push('J');
                    }
                    &['L', 'J'] => {
                        nrow.push('L');
                        nrow.push('-');
                        nrow.push('J');
                    }
                    &['L', '7'] => {
                        nrow.push('L');
                        nrow.push('-');
                        nrow.push('7');
                    }
                    &['S', '7'] => {
                        nrow.push('S');
                        nrow.push('-');
                        nrow.push('7');
                    }
                    &['S', 'J'] => {
                        nrow.push('S');
                        nrow.push('-');
                        nrow.push('J');
                    }
                    &['L', 'S'] => {
                        nrow.push('L');
                        nrow.push('-');
                        nrow.push('S');
                    }
                    &['F', 'S'] => {
                        nrow.push('F');
                        nrow.push('-');
                        nrow.push('S');
                    }
                    &[a, b] => {
                        nrow.push(a);
                        nrow.push('+');
                        nrow.push(b);
                    }
                    _ => todo!(),
                }

                last = nrow.pop().unwrap();
            }
            nrow.push(last);
            ngrid.push(nrow);
        }

        ngrid
    }

    fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let height = grid.len();
        let width = grid[0].len();

        let mut v: Vec<Vec<char>> = Vec::new();

        for w in 0..width {
            let mut row: Vec<_> = Vec::new();
            for h in 0..height {
                // println!("{:?}")
                row.insert(
                    0,
                    match grid[h][w] {
                        'F' => '7',
                        '7' => 'J',
                        'J' => 'L',
                        'L' => 'F',
                        '|' => '-',
                        '-' => '|',
                        c => c,
                    },
                );
            }
            v.push(row)
        }

        v
    }

    fn widen_xy(g: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for r in &g {
            let s = r.iter().collect::<String>();
            println!("{}", s);
        }

        let ngrid = widen(g);

        // ngrid[0][0] = 'Y';

        for r in &ngrid {
            let s = r.iter().collect::<String>();
            println!("{}", s);
        }
        // println!("");
        let tgrid = transpose(ngrid);

        for r in &tgrid {
            let s = r.iter().collect::<String>();
            println!("{}", s);
        }

        let tgrid = widen(tgrid);
        // println!("");

        for r in &tgrid {
            let s = r.iter().collect::<String>();
            println!("{}", s);
        }
        transpose(transpose(transpose(tgrid)))
    }
}
