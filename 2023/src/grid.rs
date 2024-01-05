#![allow(dead_code)]

use std::hash::Hash;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    fs,
};

#[derive(Clone)]
pub struct Grid<T> {
    pub _grid: Vec<Vec<T>>,
}

impl<T: Eq + Debug + Hash + Clone + Copy> Grid<T> {
    pub fn rows(&self) -> i32 {
        self._grid.len() as i32
    }
    pub fn cols(&self) -> i32 {
        self._grid[0].len() as i32
    }
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self { _grid: grid }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.get(x, y).is_some()
    }

    pub fn get_xy_directions_with_match(
        &self,
        x: i32,
        y: i32,
        cs: &HashSet<T>,
    ) -> Vec<(i32, i32, &T)> {
        let next: Vec<(i32, i32, &T)> = self.xy_directions(x, y);
        next.iter()
            .filter_map(|d| if cs.contains(d.2) { Some(*d) } else { None })
            .collect::<Vec<_>>()
    }

    pub fn flood(&mut self, x: i32, y: i32, cs: HashSet<T>, used_flag: T) -> bool {
        if let Some(c) = self.get(x, y) {
            if !cs.contains(c) {
                return false;
            }
        }

        let first = self
            .get_xy_directions_with_match(x, y, &cs)
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
            self.set(x, y, used_flag.clone());

            for n in self
                .get_xy_directions_with_match(x, y, &cs)
                .iter()
                .map(|(x, y, c)| (*x, *y, **c))
                .collect::<Vec<_>>()
            {
                next.push(n);
            }
        }
        return true;
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        self._grid
            .get(y as usize)
            .map(|row| row.get(x as usize).map(|c| c))
            .flatten()
    }

    pub fn transpose(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        let v = self._grid.clone();
        assert!(!v.is_empty());
        (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
    }

    // pub fn windows(&self, size: usize) {
    //     self._grid.windows(size)
    // }

    pub fn get_with_coordinates(&self, x: i32, y: i32) -> Option<(i32, i32, &T)> {
        if x < 0 || y < 0 {
            return None;
        }

        self._grid
            .get(y as usize)
            .map(|row| row.get(x as usize).map(|c| (x, y, c)))
            .flatten()
    }

    pub fn find(&self, val: &T) -> Option<(i32, i32)> {
        for (y, row) in self._grid.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if *val == *el {
                    return Some((x as i32, y as i32));
                }
            }
        }
        return None;
    }

    pub fn find_all(&self, val: &T) -> HashSet<(i32, i32)> {
        let mut res = HashSet::new();
        for (y, row) in self._grid.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if *val == *el {
                    res.insert((x as i32, y as i32));
                }
            }
        }
        return res;
    }

    pub fn xy_directions(&self, x: i32, y: i32) -> Vec<(i32, i32, &T)> {
        vec![
            self.get_with_coordinates(x - 1, y),
            self.get_with_coordinates(x + 1, y),
            self.get_with_coordinates(x, y + 1),
            self.get_with_coordinates(x, y - 1),
        ]
        .iter()
        .filter_map(|el| *el)
        .collect::<Vec<_>>()
    }

    pub fn print(&self)
    where
        T: Clone + Display,
    {
        for row in &self._grid {
            let s = row
                .iter()
                .map(|r| format!("{}", r.clone()))
                .collect::<Vec<_>>()
                .join("");
            println!("{:?}", s);
        }
        println!("")
    }

    pub fn save_to_file(&self, path: &str) {
        let g = self
            ._grid
            .iter()
            .map(|r| r.iter().map(|c| format!("{:?}", c)).collect::<String>())
            .map(|line| format!("{}\n", line.replace("\'", "")))
            .collect::<String>();
        fs::write(path, g).expect("Save file");
    }

    pub fn set(&mut self, x: i32, y: i32, val: T) -> bool {
        if self.get(x, y).is_some() {
            self._grid[y as usize][x as usize] = val;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::Grid;

    #[test]
    fn test_get() {
        let contents = fs::read_to_string("./problems/d10_sample.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let grid = Grid::new(g);

        assert_eq!(grid.get(0, 0).unwrap(), &'.');
        assert_eq!(grid.get(0, 2).unwrap(), &'S');
        assert!(grid.get(-1, -1).is_none());
        assert!(grid.get(6, 6).is_none());
    }

    #[test]
    fn test_xy_directions() {
        let contents = fs::read_to_string("./problems/d10_sample.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let grid = Grid::new(g);

        let dirs1 = grid.xy_directions(0, 0);
        println!("{:?}", dirs1);
    }

    #[test]
    fn test_update() {
        let contents = fs::read_to_string("./problems/d10_sample.txt").expect("");
        let g: Vec<Vec<char>> = contents
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut grid = Grid::new(g);

        grid.set(0, 0, 'X');

        assert_eq!(grid.get(0, 0).unwrap(), &'X');
        assert_eq!(grid.get(0, 2).unwrap(), &'S');
    }
}
