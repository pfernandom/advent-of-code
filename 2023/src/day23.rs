#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet, VecDeque},
        fs,
        rc::Rc,
    };

    use crate::grid::Grid;

    fn parse_input() -> Grid<char> {
        let contents = fs::read_to_string("./problems/d23.txt").expect("");

        Grid::new(
            contents
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn p1() {
        let grid = parse_input();
        let start = (1, 0); // c,r

        let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

        let mut queue = VecDeque::new();
        queue.push_front((start, 0, HashSet::new()));

        let mut longest_path = HashSet::new();

        while let Some(((col, row), cur_len, mut visited)) = queue.pop_back() {
            // println!(" == {} {}", row, col);
            visited.insert((col, row));

            // grid.set(col, row, 'O');

            if (row, col) == (grid.rows() - 1, grid.cols() - 2) {
                // println!("-One res: {}", cur_len);
                longest_path = visited;
                continue;
            }

            if let Some(slope) = match grid.get(col, row).unwrap() {
                '>' => Some((col + 1, row)),
                '<' => Some((col - 1, row)),
                '^' => Some((col, row - 1)),
                'v' => Some((col, row + 1)),
                _ => None,
            } {
                if !visited.contains(&(slope.0, slope.1)) {
                    queue.push_front((slope, cur_len + 1, visited.clone()));
                }
                continue;
            }

            for (col, row, _) in grid
                .get_xy_directions_with_match(col, row, &expected)
                .iter()
                .filter(|(col, row, _)| !visited.contains(&(*col, *row)))
                .collect::<Vec<_>>()
            {
                queue.push_front(((*col, *row), cur_len + 1, visited.clone()));
            }
        }

        assert!(!longest_path.is_empty());
        println!("longest_path: {}", longest_path.len() - 1);

        // for (col, row) in longest_path {
        //     grid.set(col, row, 'O');
        // }

        // grid.print();
    }

    #[derive(Eq)]
    struct Node {
        row: i32,
        col: i32,
        dist: i32,
        visited: HashSet<(i32, i32)>,
        path: VecDeque<(i32, i32)>,
        last_cross_roads: Option<(i32, i32)>,
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            // self.visited.len() == other.visited.len()
            self.dist == other.dist
        }
    }

    impl PartialOrd for Node {
        // fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //     // self.visited.len().partial_cmp(&other.visited.len())
        //     match self.dist.partial_cmp(&other.dist) {
        //         Some(ord) => match ord {
        //             std::cmp::Ordering::Equal => {
        //                 self.dist_to_start().partial_cmp(&other.dist_to_start())
        //             }
        //             o => Some(o),
        //         },
        //         None => None,
        //     }
        // }

        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.dist_to_start().partial_cmp(&other.dist_to_start())
        }
    }

    impl Ord for Node {
        // fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //     // self.visited.len().cmp(&other.visited.len())
        //     match self.dist.cmp(&other.dist) {
        //         std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        //         std::cmp::Ordering::Equal => self.dist_to_start().cmp(&other.dist_to_start()),
        //         std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        //     }
        // }

        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.dist_to_start().cmp(&other.dist_to_start())
        }
    }

    impl Node {
        fn new(col: i32, row: i32, visited: &HashSet<(i32, i32)>) -> Self {
            Self {
                row,
                col,
                dist: 0,
                visited: visited.clone(),
                path: VecDeque::new(),
                last_cross_roads: None,
            }
        }

        fn new_path(&self, col: i32, row: i32, last_cross_roads: Option<(i32, i32)>) -> Self {
            let mut path = VecDeque::new();
            path.extend(&self.path);

            path.push_back((col, row));

            let last_cross_roads = last_cross_roads.or(self.last_cross_roads);

            Self {
                row,
                col,
                visited: self.visited.clone(),
                dist: self.dist + 1,
                path,
                last_cross_roads,
            }
        }

        fn prev(&self) -> Option<(i32, i32)> {
            if self.path.len() < 2 {
                None
            } else {
                self.path.get(self.path.len() - 2).cloned()
            }
        }

        fn is_prev(&self, col: i32, row: i32) -> bool {
            self.prev().map(|p| p == (col, row)).unwrap_or(false)
        }

        fn add_visited(&mut self, col: i32, row: i32) {
            self.visited.insert((col, row));
        }

        fn dist_to_start(&self) -> i32 {
            (self.col.abs_diff(1) + self.row.abs_diff(0)) as i32
        }

        fn clear_alley(&mut self, grid: &mut Grid<char>) {
            let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

            while let Some((col, row)) = self.path.pop_back() {
                // match self.last_cross_roads {
                //     Some((cc, cr)) => {
                //         if (cc, cr) == (col, row) {
                //             break;
                //         }
                //     }
                //     None => {}
                // }

                let dirs = grid
                    .get_xy_directions_with_match(col, row, &expected)
                    .iter()
                    .map(|t| (t.0, t.1, *t.2))
                    .collect::<Vec<_>>();

                if dirs
                    .iter()
                    .all(|(col, row, _)| self.visited.contains(&(*col, *row)))
                {
                    grid.set(col, row, 'X');
                } else {
                    break;
                }
            }
        }
    }

    // fn traverse(
    //     col: i32,
    //     row: i32,
    //     grid: &Grid<char>,
    //     visited: &mut HashSet<(i32, i32)>,
    // ) -> Option<i32> {
    //     let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

    //     if (col, row) == (grid.cols() - 2, grid.rows() - 1) {
    //         println!("F");
    //         return Some(0);
    //     }
    //     visited.insert((col, row));

    //     let dirs = grid
    //         .get_xy_directions_with_match(col, row, &expected)
    //         .iter()
    //         .map(|t| (t.0, t.1, *t.2))
    //         .collect::<Vec<_>>();
    //     let allowed_neighs = dirs
    //         .iter()
    //         .filter(|(col, row, _)| !visited.contains(&(*col, *row)))
    //         .filter_map(|(col, row, _)| traverse(*col, *row, grid, &mut visited.clone()))
    //         .collect::<Vec<_>>();

    //     visited.remove(&(col, row));
    //     if allowed_neighs.is_empty() {
    //         return None;
    //     }

    //     let mut m = 0;

    //     for n in allowed_neighs {
    //         m = m.max(n);
    //     }

    //     Some(m + 1)
    // }

    fn get_reachable(col: i32, row: i32, grid: &Grid<char>) -> HashMap<(i32, i32), i32> {
        let mut m = HashMap::new();
        let mut queue = VecDeque::new();
        // let mut visited = Hah
        queue.push_front((col, row));
        let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

        while let Some((col, row)) = queue.pop_back() {
            m.insert((col, row), 0);

            if (row, col) == (0, 1) {
                continue;
            }

            let dirs = grid
                .get_xy_directions_with_match(col, row, &expected)
                .iter()
                .map(|t| (t.0, t.1, *t.2))
                .collect::<Vec<_>>();
            let allowed_neighs = dirs
                .iter()
                .filter(|(col, row, _)| !m.contains_key(&(*col, *row)))
                .collect::<Vec<_>>();

            for (col, row, _) in allowed_neighs {
                queue.push_front((*col, *row));
            }
        }
        m
    }

    #[derive(Clone)]
    struct GNode {
        val: (i32, i32),
        children: Vec<Rc<RefCell<GNode>>>,
        dist: i32,
    }

    fn find_crossroads(
        start: (i32, i32),
        end: (i32, i32),
        grid: &Grid<char>,
    ) -> HashSet<(i32, i32)> {
        let m = GNode {
            val: (start.0, start.1),
            children: Vec::new(),

            dist: 0,
        };
        let mut crossroads = HashSet::new();
        let mut queue: VecDeque<Rc<RefCell<GNode>>> = VecDeque::new();
        // let mut visited = Hah
        let m = Rc::new(RefCell::new(m));
        queue.push_front(m.clone());
        let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

        let mut visited = HashSet::new();

        while let Some(node) = queue.pop_back() {
            let (col, row) = node.borrow().val;

            let k = (node.borrow().val.0, node.borrow().val.1);
            // m.insert((col, row), 0);
            if visited.contains(&k) {
                continue;
            }
            visited.insert(k);

            if (col, row) == end {
                print!("end");
                continue;
            }

            let dirs = grid
                .get_xy_directions_with_match(col, row, &expected)
                .iter()
                .map(|t| (t.0, t.1, *t.2))
                .collect::<Vec<_>>();
            let allowed_neighs = dirs
                .iter()
                // .filter(|(col, row, _)| !m.contains_key(&(*col, *row)))
                .collect::<Vec<_>>();

            if allowed_neighs.len() > 2 {
                crossroads.insert((col, row));
            }

            for (col, row, _) in allowed_neighs {
                let c = GNode {
                    val: (*col, *row),
                    children: Vec::new(),
                    dist: 0,
                };

                let n = Rc::new(RefCell::new(c));

                queue.push_front(n.clone());

                node.borrow_mut().children.push(n.clone());
            }
        }
        crossroads
    }

    fn reduce_graph(
        start: (i32, i32),
        end: (i32, i32),
        grid: &Grid<char>,
        crossroads: &HashSet<(i32, i32)>,
    ) -> HashMap<(i32, i32), HashSet<((i32, i32), i32)>> {
        let mut results: HashMap<(i32, i32), HashSet<((i32, i32), i32)>> = HashMap::new();

        let mut queue: VecDeque<((i32, i32), (i32, i32), i32, HashSet<(i32, i32)>)> =
            VecDeque::new();

        let mut found = HashSet::new();

        for c in crossroads {
            queue.push_front((c.clone(), c.clone(), 0, HashSet::new()))
        }

        queue.push_front((start, start, 0, HashSet::new()));

        let expected = HashSet::from(['.', 'v', '>', '<', '^', 'O']);

        while let Some((mut from, (col, row), dist, mut visited)) = queue.pop_back() {
            let k = (col, row);

            if visited.contains(&k) {
                continue;
            }
            visited.insert(k);

            if k == end {
                results
                    .entry(from)
                    .or_insert(HashSet::new())
                    .insert((k, dist));
                continue;
            }

            if crossroads.contains(&(col, row)) && from != (col, row) {
                results
                    .entry(from)
                    .or_insert(HashSet::new())
                    .insert((k, dist));
                from = k;

                found.insert(from);

                continue;
            }

            let dirs = grid
                .get_xy_directions_with_match(col, row, &expected)
                .iter()
                .map(|t| (t.0, t.1, *t.2))
                .collect::<Vec<_>>();
            let allowed_neighs = dirs.iter().collect::<Vec<_>>();

            for (col, row, _) in allowed_neighs {
                queue.push_front((from, (*col, *row), dist + 1, visited.clone()));
            }
        }
        // }
        results
    }

    fn print_path(root: Rc<RefCell<GNode>>, level: usize) -> Option<i32> {
        let r = root.borrow();
        let sp = vec![" "; level].join("");
        println!("{}- {:?} dist:{}", sp, r.val, r.dist);
        let res = r.dist;
        let mut vmax = 0;

        if r.val == (21, 22) {
            return Some(1);
        }

        if r.children.is_empty() {
            return None;
        }

        let childs = r
            .children
            .iter()
            .map(|c| print_path(c.clone(), level + 1))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap());

        for c in childs {
            vmax = vmax.max(c);
        }
        Some(res + vmax)
    }

    fn traverse_reduced(
        start: (i32, i32),
        end: (i32, i32),
        reduced: &HashMap<(i32, i32), HashSet<((i32, i32), i32)>>,
    ) -> i32 {
        let mut queue = VecDeque::new();

        queue.push_front((start, start, 0, HashSet::new()));

        let mut r = 0;

        while let Some((from, node, dist, mut visited)) = queue.pop_back() {
            println!("{:?}, {:?}", from, node);
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            if node == end {
                r = r.max(dist);
                continue;
            }
            match reduced.get(&node) {
                Some(next) => {
                    for (n, local_dist) in next {
                        if *n != from {
                            queue.push_front((node, n.clone(), dist + local_dist, visited.clone()));
                        }
                    }
                }
                None => {}
            };
        }

        r
    }

    fn traverse_reduced2(
        node: (i32, i32),
        end: (i32, i32),
        visited: &mut HashSet<(i32, i32)>,
        reduced: &HashMap<(i32, i32), HashSet<((i32, i32), i32)>>,
    ) -> i32 {
        if node == end {
            return 0;
        }
        let mut m = i32::MIN;
        match reduced.get(&node) {
            Some(next) => {
                for (n, local_dist) in next {
                    if visited.contains(&node) {
                        continue;
                    }
                    visited.insert(node);
                    m = m.max(traverse_reduced2(n.clone(), end, visited, reduced) + local_dist);
                    visited.remove(&node);
                }
            }
            None => {}
        };

        m
    }

    #[test]
    fn p2() {
        let grid: Grid<char> = parse_input();
        let start = (1, 0); // c,r
        let end = (grid.cols() - 2, grid.rows() - 1); // c,r

        let crossroads = find_crossroads(start, end, &grid);
        println!("crossroads: {:?}", crossroads);
        let g = reduce_graph(start, end, &grid, &crossroads);
        // let r = print_path(g.clone(), 0);
        println!("g={:?}", g);

        for (k, v) in &g {
            println!("=> {:?}", k);
            for vv in v {
                println!(" - {:?}, ({})", vv, vv.0 == end)
            }
        }
        // TODO: Too slow!
        println!(
            "r: {}",
            traverse_reduced2(start, end, &mut HashSet::new(), &g)
        );
    }
}
