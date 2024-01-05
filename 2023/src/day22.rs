#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        fs,
    };

    fn get_bricks() -> VecDeque<Brick> {
        let contents = fs::read_to_string("./problems/d22.txt").expect("");

        let mut names = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "Y", "Z",
        ]
        .iter()
        .cycle();

        let mut ids = (100..1000000).cycle();

        let mut bricks = contents
            .lines()
            .map(|l| l.split_once("~").unwrap())
            .map(|(b1, b2)| {
                (
                    b1.split(",")
                        .map(|e| i32::from_str_radix(e, 10).unwrap())
                        .collect::<Vec<_>>(),
                    b2.split(",")
                        .map(|e| i32::from_str_radix(e, 10).unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(b1, b2)| {
                Brick::new(
                    names.next().unwrap().to_string(),
                    ids.next().unwrap(),
                    (b1[0], b1[1], b1[2]),
                    (b2[0], b2[1], b2[2]),
                )
            })
            .collect::<Vec<_>>();

        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
        let bricks = VecDeque::from(bricks);

        // let m = bricks
        //     .iter()
        //     .map(|b| b.start.z)
        //     .reduce(|a, z| if a < z { a } else { z })
        //     .unwrap()
        //     - 1;

        // println!("min: {}", m);

        // bricks.iter_mut().for_each(|b| {
        //     b.start.z -= m;
        //     b.end.z -= m;
        // });

        bricks
    }

    #[derive(Debug, Clone)]
    struct Coord {
        x: i32,
        y: i32,
        z: i32,
    }

    impl Coord {
        fn new(coords: (i32, i32, i32)) -> Self {
            Self {
                x: coords.0,
                y: coords.1,
                z: coords.2,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Brick {
        id: i32,
        label: String,
        start: Coord,
        end: Coord,
    }

    impl Brick {
        fn new(label: String, id: i32, start: (i32, i32, i32), end: (i32, i32, i32)) -> Self {
            Self {
                label,
                id,
                start: Coord::new(start),
                end: Coord::new(end),
            }
        }

        fn from_input(id: i32, start: (i32, i32, i32), end: (i32, i32, i32)) -> Self {
            Self {
                label: String::new(),
                id,
                start: Coord::new(start),
                end: Coord::new(end),
            }
        }

        fn z_len(&self) -> i32 {
            self.end.z.abs_diff(self.start.z) as i32 + 1
        }

        fn floor_distance(&self) -> i32 {
            (1 as i32).abs_diff(self.start.z.min(self.end.z)) as i32
        }

        fn get_dim(&self, dim: usize) -> (i32, i32) {
            if dim == 0 {
                (self.start.x, self.end.x)
            } else if dim == 1 {
                (self.start.y, self.end.y)
            } else if dim == 2 {
                (self.start.z, self.end.z)
            } else {
                unreachable!()
            }
        }

        fn overlaps_dim(&self, dim: usize, other: &Self) -> bool {
            let (s_s, s_end) = self.get_dim(dim);
            let (o_s, o_end) = other.get_dim(dim);
            let x = s_s..s_end + 1;

            x.contains(&o_s) || x.contains(&o_end)
        }

        fn overlaps_x(&self, other: &Self) -> bool {
            self.overlaps_dim(0, other)
        }

        fn overlaps_y(&self, other: &Self) -> bool {
            self.overlaps_dim(1, other)
        }

        fn overlaps_z(&self, other: &Self) -> bool {
            self.overlaps_dim(2, other)
        }

        fn overlaps_with(&self, other: &Self) -> bool {
            let mut overlaps = 0;
            if self.overlaps_x(other) || other.overlaps_x(&self) {
                // println!("{} overlaps in x with {}", self.label, other.label);
                overlaps += 1;
            }
            if self.overlaps_y(other) || other.overlaps_y(&self) {
                // println!("{} overlaps in y with {}", self.label, other.label);
                overlaps += 1;
            }
            if self.overlaps_z(other) || other.overlaps_z(&self) {
                // println!("{} overlaps in z with {}", self.label, other.label);
                overlaps += 1;
            }

            // if overlaps == 3 {
            //     println!("{} overlaps in y with {}", self.label, other.label);
            // }
            overlaps == 3
        }

        fn top(&self) -> i32 {
            assert!(self.start.z <= self.end.z, "{:?}", self);
            self.end.z
        }

        fn supports(&self, other: &Self) -> bool {
            let mut s = self.clone();
            s.update_dim(2, 1);
            s.overlaps_with(other)
        }

        // fn supports(&self, others: Vec<&Self>) -> Vec<&Self> {
        //     for other in others {}
        // }

        fn update_dim(&mut self, dim: usize, delta: i32) {
            if dim == 0 {
                self.start.x += delta;
                self.end.x += delta;
            } else if dim == 1 {
                self.start.y += delta;
                self.end.y += delta;
            } else if dim == 2 {
                self.start.z += delta;
                self.end.z += delta;
            } else {
                unreachable!()
            }
        }

        fn contains_int_y(&self, x: i32, z: i32) -> bool {
            (self.start.x..self.end.x + 1).contains(&x)
                && (self.start.z..self.end.z + 1).contains(&z)
        }

        fn contains_int_x(&self, y: i32, z: i32) -> bool {
            (self.start.y..self.end.y + 1).contains(&y)
                && (self.start.z..self.end.z + 1).contains(&z)
        }
    }

    fn print_y(bricks: &VecDeque<Brick>) {
        println!("====== x vs z =======");
        let mut max_x = 0;
        let mut max_z = 0;
        for b in bricks {
            max_x = max_x.max(b.end.x);
            max_z = max_z.max(b.top());
        }
        max_z += 1;
        max_x += 1;

        let mut matrix = vec![vec!["."; max_x as usize]; max_z as usize];

        for z in 0..max_z {
            for x in 0..max_x {
                // let mut printed = false;
                for b in bricks {
                    if b.contains_int_y(x, max_z - z - 1) {
                        // print!("{}", b.label);
                        if matrix[(max_z - z - 1) as usize][x as usize] != "." {
                            matrix[(max_z - z - 1) as usize][x as usize] = "X";
                        } else {
                            matrix[(max_z - z - 1) as usize][x as usize] = b.label.as_str();
                        }
                    }
                }
            }
        }

        for row in matrix.iter().rev() {
            println!("{}", row.join(""));
        }
    }

    fn print_x(bricks: &VecDeque<Brick>) {
        println!("====== y vs z =======");
        let mut max_y = 0;
        let mut max_z = 0;
        for b in bricks {
            max_y = max_y.max(b.end.y);
            max_z = max_z.max(b.top());
        }
        max_z += 1;
        max_y += 1;

        let mut matrix = vec![vec!["."; max_y as usize]; max_z as usize];

        for z in 0..max_z {
            for y in 0..max_y {
                // let mut printed = false;
                for b in bricks {
                    if b.contains_int_x(y, max_z - z - 1) {
                        // print!("{}", b.label);
                        if matrix[(max_z - z - 1) as usize][y as usize] != "." {
                            matrix[(max_z - z - 1) as usize][y as usize] = "X";
                        } else {
                            matrix[(max_z - z - 1) as usize][y as usize] = b.label.as_str();
                        }
                    }
                }
            }
        }

        for row in matrix.iter().rev() {
            println!("{}", row.join(""));
        }
    }

    #[test]
    fn p1() {
        let bricks = get_bricks();

        let total_to_remove = solve_p1(bricks).0;

        println!("Part A: {}", total_to_remove.len());
    }

    fn solve_p1(
        mut bricks: VecDeque<Brick>,
    ) -> (
        HashSet<i32>,
        VecDeque<Brick>,
        HashMap<i32, Vec<i32>>,
        HashMap<i32, Vec<i32>>,
    ) {
        let mut updated_bricks: VecDeque<Brick> = VecDeque::new();

        let mut on_top = HashMap::new();
        let mut bricks_below = HashMap::new();

        while let Some(b1) = bricks.pop_front().as_mut() {
            println!("Brick {}", b1.label);
            println!("Floor dist: {}", b1.floor_distance());
            bricks_below.entry(b1.id).or_insert(Vec::new());

            while b1.floor_distance() > 0 {
                b1.update_dim(2, -1);
                let base = updated_bricks
                    .iter()
                    .filter(|front| front.overlaps_with(b1))
                    .collect::<Vec<_>>();

                // TODO: Speed things up
                // let empty = Vec::new();
                // let base = z_bricks
                //     .get(&b1.start.z)
                //     .unwrap_or(&empty)
                //     .iter()
                //     .filter(|front| front.overlaps_with(b1))
                //     .collect::<Vec<_>>();

                if !base.is_empty() {
                    b1.update_dim(2, 1);
                    for b2 in base {
                        assert!(b2.supports(&b1));
                        bricks_below.entry(b1.id).or_insert(Vec::new()).push(b2.id);
                        on_top.entry(b2.id).or_insert(Vec::new()).push(b1.id);
                    }
                    break;
                }
            }

            println!("Floor dist after: {}", b1.floor_distance());
            updated_bricks.push_front(b1.clone());
        }

        print_y(&updated_bricks);
        println!("");
        print_x(&updated_bricks);

        println!("Supporting:");

        let mut all_supports: HashSet<&i32> = HashSet::new();
        let mut single_supports: HashSet<&i32> = HashSet::new();
        for (_, v) in &bricks_below {
            all_supports.extend(v);
            if v.len() == 1 {
                single_supports.extend(v);
            }
        }

        let multi_supports = bricks_below
            .values()
            .filter(|s| s.len() > 1)
            .flat_map(|s| s)
            .filter(|b| !single_supports.contains(b))
            .collect::<HashSet<_>>();

        let no_supports = updated_bricks
            .iter()
            .map(|b| b.id)
            .filter(|b| !all_supports.contains(b))
            .collect::<HashSet<_>>();

        let mut res = HashSet::new();

        res.extend(multi_supports);
        res.extend(no_supports);
        (res, updated_bricks, on_top, bricks_below)
    }

    #[derive(Debug, Clone)]
    struct GraphNodeMap {
        data: HashMap<i32, GraphNode>,
    }

    impl GraphNodeMap {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }

        fn delete_node(&mut self, i: i32) -> i32 {
            // let ch = self.data.get(&i).unwrap();
            let mut r = 0;
            // self.data.remove(&i);

            let mut to_remove = Vec::new();

            for d in self.data.values_mut() {
                d.parents.remove(&i);
                if d.parents.is_empty() {
                    r += 1;
                    d.parents.insert(-2);
                    to_remove.push(d.data);
                }
                d.children.remove(&i);
            }

            for i in to_remove {
                r += self.delete_node(i);
            }

            // println!("r:{}", r)
            r
        }

        fn traverse(&mut self, from: i32, i: i32, on_top: &HashMap<i32, Vec<i32>>) {
            let node = self.data.entry(i).or_insert(GraphNode::new(i));
            node.append_parent(from);
            match on_top.get(&i) {
                Some(bot) => {
                    if bot.is_empty() {
                        node.append_child(-1);
                    } else {
                        node.append_children(bot);
                    }

                    for b in bot {
                        self.traverse(i, *b, on_top)
                    }
                }
                None => {}
            }
        }
    }

    #[derive(Debug, Clone)]
    struct GraphNode {
        parents: HashSet<i32>,
        children: HashSet<i32>,
        data: i32,
    }

    impl GraphNode {
        fn new(data: i32) -> Self {
            Self {
                data,
                parents: HashSet::new(),
                children: HashSet::new(),
            }
        }

        fn append_child(&mut self, data: i32) {
            self.children.insert(data);
        }

        fn append_children(&mut self, data: &Vec<i32>) {
            for d in data {
                self.children.insert(*d);
            }
        }

        fn append_parent(&mut self, data: i32) {
            self.parents.insert(data);
        }
    }

    #[test]
    fn p2() {
        let bricks = get_bricks();

        let (to_remove, updated_bricks, on_top, bricks_below) = solve_p1(bricks);

        let to_not_remove = updated_bricks
            .iter()
            .filter(|b| !to_remove.contains(&b.id))
            .collect::<Vec<_>>();
        println!("to_remove= {:?}", to_remove);
        println!("to_not_remove={:?}", to_not_remove);

        let roots = bricks_below
            .iter()
            .filter_map(|(k, v)| if v.len() == 0 { Some(*k) } else { None })
            .collect::<Vec<_>>();

        println!("roots:{:?}", roots);

        let mut node_map = GraphNodeMap::new();
        for r in roots {
            node_map.traverse(-1, r, &on_top);
        }

        println!("{:?}", node_map);

        let mut res = 0;
        for nr in to_not_remove {
            res += node_map.clone().delete_node(nr.id);
        }

        println!(
            "{:?}",
            node_map
                .data
                .values()
                .filter(|v| v.parents.len() == 1 && v.parents.contains(&-2))
                .count()
        );

        // println!("{:?}", node_map);
        println!("res:{}", res);
    }

    #[test]
    fn test_z_len() {
        let b = Brick::from_input(1, (0, 0, 1), (0, 0, 10));

        assert_eq!(b.z_len(), 10);
        let b = Brick::from_input(2, (0, 0, 1), (0, 0, 1));

        assert_eq!(b.z_len(), 1);
    }

    #[test]
    fn test_z_len_2() {
        let b = Brick::from_input(1, (0, 0, 1), (0, 0, 1));

        assert_eq!(b.floor_distance(), 0);

        let b = Brick::from_input(2, (0, 0, 5), (0, 0, 10));

        assert_eq!(b.floor_distance(), 4)
    }

    #[test]
    fn test_overlaps_with() {
        let b1 = Brick::from_input(1, (0, 0, 1), (0, 0, 1));
        let b2 = Brick::from_input(2, (0, 0, 1), (0, 0, 1));

        assert!(b1.overlaps_with(&b2));

        let b1 = Brick::from_input(1, (0, 0, 1), (0, 0, 1));
        let b2 = Brick::from_input(2, (0, 0, 2), (0, 0, 2));

        assert!(!b1.overlaps_with(&b2));

        let b1 = Brick::from_input(1, (0, 0, 1), (0, 1, 1));
        let b2 = Brick::from_input(2, (0, 0, 2), (0, 0, 2));

        assert!(!b1.overlaps_with(&b2));

        let b1 = Brick::from_input(1, (0, 0, 1), (0, 0, 1));
        let b2 = Brick::from_input(2, (0, 1, 1), (0, 1, 1));

        assert!(!b1.overlaps_with(&b2));
    }
}
