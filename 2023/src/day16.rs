#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use crate::grid::Grid;

    #[test]
    fn pre_tests() {
        let contents = fs::read_to_string("./problems/d16.txt").expect("");
        let grid: Grid<char> = Grid::new(
            contents
                .split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            Direction::ToDown((7, 1)).next(&grid).unwrap(),
            NewBeam::Split((Direction::ToLeft((7, 0)), Direction::ToRight((7, 2))))
        )
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d16.txt").expect("");
        let grid: Grid<char> = Grid::new(
            contents
                .split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        grid.print();

        let (_, res) = draw_splitters(grid, Direction::ToRight((0, 0)));
        println!("res: {}", res);
    }

    #[test]
    fn pre_p2() {
        let contents = fs::read_to_string("./problems/d16.txt").expect("");
        let mut grid: Grid<char> = Grid::new(
            contents
                .split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        grid.print();

        let wr = grid._grid.len();
        let wc = grid._grid[0].len();
        let last_row = (wr - 1) as i32;
        let last_col = (wc - 1) as i32;

        for c in 0..last_col + 1 {
            grid.set(c, 0, 'V');
            grid.set(c, last_row, 'A');
        }

        for r in 0..last_row + 1 {
            grid.set(0, r, '>');
            grid.set(last_col, r, '<');
        }

        grid.print();
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d16.txt").expect("");
        let grid: Grid<char> = Grid::new(
            contents
                .split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        grid.print();

        let wr = grid._grid.len();
        let wc = grid._grid[0].len();
        let last_row = (wr - 1) as i32;
        let last_col = (wc - 1) as i32;

        let mut pos_res = Vec::new();
        // let mut res = 0;

        for c in 0..last_col + 1 {
            let (_, mut res) = draw_splitters(grid.clone(), Direction::ToDown((0, c)));
            pos_res.push(res);
            (_, res) = draw_splitters(grid.clone(), Direction::ToUp((last_row, c)));
            pos_res.push(res);
        }

        for r in 0..last_row + 1 {
            let (_, mut res) = draw_splitters(grid.clone(), Direction::ToRight((r, 0)));
            pos_res.push(res);
            (_, res) = draw_splitters(grid.clone(), Direction::ToLeft((r, last_col)));
            pos_res.push(res);
        }

        let res = pos_res
            .iter()
            .reduce(|accum, item| if accum >= item { accum } else { item });

        println!("res: {:?}", res);
        println!("pos_res: {:?}", pos_res);
    }

    fn draw_splitters(mut grid: Grid<char>, dir: Direction) -> (Grid<char>, i32) {
        let mut beams = Vec::new();

        let mut energ = grid.clone();

        beams.push(dir);

        fn draw_dir(grid: &mut Grid<char>, energ: &mut Grid<char>, b: &Direction) {
            let ((r, c), new_ch) = match b {
                Direction::ToRight(c) => (*c, '>'),
                Direction::ToLeft(c) => (*c, '<'),
                Direction::ToUp(c) => (*c, 'A'),
                Direction::ToDown(c) => (*c, 'V'),
            };

            match grid.get(c, r) {
                Some(ch) => {
                    if *ch == '.' {
                        grid.set(c, r, new_ch);
                    }
                    energ.set(c, r, 'X');
                }
                None => {}
            };

            // grid.print();
            // energ.print();
        }

        let mut count = 0;

        let mut found = HashSet::new();

        while !&beams.is_empty() {
            let next_dir = beams.pop();

            next_dir.map(|dir| {
                match dir.next(&grid) {
                    Some(nb) => match nb {
                        NewBeam::Single(beam) => {
                            if !found.contains(&beam) {
                                found.insert(beam.clone());
                                beams.push(beam);
                            }
                        }
                        NewBeam::Split((b1, b2)) => {
                            if !found.contains(&b1) {
                                found.insert(b1.clone());
                                beams.push(b1);
                            }
                            if !found.contains(&b2) {
                                found.insert(b2.clone());
                                beams.push(b2);
                            }
                        }
                    },
                    None => {}
                }
                draw_dir(&mut grid, &mut energ, &dir);
                if count % 101 == 1 {
                    grid.print();
                }
                count += 1;
            });
        }
        grid.print();

        let energ_count = energ
            ._grid
            .iter()
            .flat_map(|l| l)
            .map(|r| if *r == 'X' { 1 } else { 0 })
            .sum::<i32>();

        // println!("energ_count={}", energ_count);

        (grid, energ_count)
    }

    type Coord = (i32, i32);

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Direction {
        ToRight(Coord),
        ToLeft(Coord),
        ToUp(Coord),
        ToDown(Coord),
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]

    enum NewBeam {
        Single(Direction),
        Split((Direction, Direction)),
    }

    impl Direction {
        fn new_up(c: &Coord) -> Self {
            Direction::ToUp((c.0 - 1, c.1))
        }

        fn new_down(c: &Coord) -> Self {
            Direction::ToDown((c.0 + 1, c.1))
        }

        fn new_left(c: &Coord) -> Self {
            Direction::ToLeft((c.0, c.1 - 1))
        }

        fn new_right(c: &Coord) -> Self {
            Direction::ToRight((c.0, c.1 + 1))
        }

        fn next(&self, grid: &Grid<char>) -> Option<NewBeam> {
            let n: Option<NewBeam> = match self {
                Direction::ToRight(c) => match self._move(c, grid) {
                    Some(ch) => {
                        if ch == '>' || ch == '<' {
                            return None;
                        }

                        Some(match ch {
                            '|' => NewBeam::Split((Direction::new_up(c), Direction::new_down(c))),
                            '-' => NewBeam::Single(Direction::new_right(c)),
                            '\\' => NewBeam::Single(Direction::new_down(c)),
                            '/' => NewBeam::Single(Direction::new_up(c)),
                            '.' => NewBeam::Single(Direction::new_right(c)),
                            _ => {
                                // println!("WARN1 {}", x);
                                NewBeam::Single(Direction::new_right(c))
                            }
                        })
                    }
                    None => None,
                },
                Direction::ToLeft(c) => match self._move(c, grid) {
                    Some(ch) => {
                        if ch == '<' || ch == '>' {
                            return None;
                        }
                        Some(match ch {
                            '|' => NewBeam::Split((Direction::new_up(c), Direction::new_down(c))),
                            '-' => NewBeam::Single(Direction::new_left(c)),
                            '\\' => NewBeam::Single(Direction::new_up(c)),
                            '/' => NewBeam::Single(Direction::new_down(c)),
                            '.' => NewBeam::Single(Direction::new_left(c)),
                            _ => {
                                // println!("WARN2 {}", x);
                                NewBeam::Single(Direction::new_left(c))
                            }
                        })
                    }
                    None => None,
                },
                Direction::ToUp(c) => match self._move(c, grid) {
                    Some(ch) => {
                        if ch == 'A' || ch == 'V' {
                            return None;
                        }
                        Some(match ch {
                            '|' => NewBeam::Single(Direction::new_up(c)),
                            '-' => {
                                NewBeam::Split((Direction::new_left(c), Direction::new_right(c)))
                            }
                            '\\' => NewBeam::Single(Direction::new_left(c)),
                            '/' => NewBeam::Single(Direction::new_right(c)),
                            '.' => NewBeam::Single(Direction::new_up(c)),
                            _ => {
                                // println!("WARN3 {}", x);
                                NewBeam::Single(Direction::new_up(c))
                            }
                        })
                    }
                    None => None,
                },
                Direction::ToDown(c) => match self._move(c, grid) {
                    Some(ch) => {
                        if ch == 'V' || ch == 'A' {
                            return None;
                        }

                        Some(match ch {
                            '|' => NewBeam::Single(Direction::new_down(c)),
                            '-' => {
                                NewBeam::Split((Direction::new_left(c), Direction::new_right(c)))
                            }
                            '\\' => NewBeam::Single(Direction::new_right(c)),
                            '/' => NewBeam::Single(Direction::new_left(c)),
                            '.' => NewBeam::Single(Direction::new_down(c)),
                            _ => {
                                // println!("WARN4 {}", x);
                                NewBeam::Single(Direction::new_down(c))
                            }
                        })
                    }
                    None => None,
                },
            };
            n
        }

        fn _move(&self, (r, c): &Coord, grid: &Grid<char>) -> Option<char> {
            grid.get(*c as i32, *r as i32).map(|c| *c)
        }

        fn get_x_y(&self) -> &Coord {
            match self {
                Direction::ToRight(c) => c,
                Direction::ToLeft(c) => c,
                Direction::ToUp(c) => c,
                Direction::ToDown(c) => c,
            }
        }
    }
}
