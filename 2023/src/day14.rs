#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::{
        collections::{hash_map::DefaultHasher, HashMap},
        fs,
        hash::{Hash, Hasher},
    };

    const TOTAL_CYCLES: i64 = 1000000000;

    #[derive(Clone)]
    struct Grid(Vec<Vec<char>>);

    impl Grid {
        fn rows(&self) -> &Vec<Vec<char>> {
            &self.0
        }

        fn rotate_right(&self) -> Grid {
            let height = self.0.len();
            let width = self.0[0].len();

            let mut v: Vec<Vec<char>> = Vec::new();

            for w in 0..width {
                let mut row: Vec<_> = Vec::new();
                for h in 0..height {
                    // println!("{:?}")
                    row.insert(0, self.0[h][w]);
                }
                v.push(row)
            }

            Grid(v)
        }

        fn rotate_left(&self) -> Grid {
            let height = self.0.len();
            let width = self.0[0].len();

            let mut v: Vec<Vec<char>> = Vec::new();

            for w in (0..width).rev() {
                let mut row: Vec<_> = Vec::new();
                for h in (0..height).rev() {
                    // println!("{:?}")
                    row.insert(0, self.0[h][w]);
                }
                v.push(row)
            }

            Grid(v)
        }

        fn print(&self) {
            println!();
            for row in &self.0 {
                let srow = row.iter().collect::<String>();
                println!("{}", srow);
            }
        }

        fn cols(&self) -> Vec<Vec<char>> {
            let height = self.cols_len();
            let width = self.rows_len();

            let mut v: Vec<Vec<char>> = Vec::new();

            for w in 0..width {
                let mut row: Vec<_> = Vec::new();
                for h in 0..height {
                    // println!("{:?}")
                    row.insert(0, self.0[h][w]);
                }
                v.push(row)
            }

            v
        }

        fn rows_len(&self) -> usize {
            self.0.len()
        }

        fn cols_len(&self) -> usize {
            self.0.get(0).map(|first| first.len()).unwrap_or(0)
        }

        fn map<F, B>(&self, map_fn: &mut F) -> Vec<Vec<B>>
        where
            F: FnMut(usize, usize, &char) -> B,
        {
            self.0
                .iter()
                .enumerate()
                .map(|(irow, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(icol, ch)| map_fn(irow, icol, ch))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        fn set(&mut self, row: usize, col: usize, c: char) {
            self.0[row][col] = c;
        }
    }

    fn parse_input() -> Grid {
        let contents = fs::read_to_string("./problems/d14.txt").expect("");

        Grid(
            contents
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn p1() {
        let inp = parse_input();

        let mut t = inp.rotate_right();

        // t.print();

        slide_rocks(&mut t);
        // println!();
        let t = t.rotate_left();
        // t.print();

        let mut total_sum = 0;
        for (r, row) in t.rows().iter().enumerate() {
            total_sum += row
                .iter()
                .filter_map(|c| {
                    if *c == 'O' {
                        Some(t.rows_len() - r)
                    } else {
                        None
                    }
                })
                .sum::<usize>();
        }

        println!("res: {}", total_sum);
    }

    fn slide_rocks(t: &mut Grid) {
        let mut i = t.cols_len() - 1;
        let mut j = i;

        for k in 0..t.rows_len() {
            while i > 0 {
                while i > 0 && j > 0 && t.0[k][i] != '.' {
                    i -= 1;
                    j -= 1;
                }

                let mut skip = false;
                while j > 0 && t.0[k][j] != 'O' {
                    if t.0[k][j] == '#' {
                        i = j;
                        // j -= 1;
                        skip = true;
                        break;
                    } else {
                        j -= 1;
                    }
                }

                if skip {
                    continue;
                }

                if j == 0 && (t.0[k][j] == '#' || t.0[k][j] == '.') {
                    break;
                }

                if i == 0 && j == 0 {
                    break;
                }

                if t.0[k][j] != 'O' {
                    println!("{} ({}),{} ({})", i, t.0[k][i], j, t.0[k][j]);
                    panic!("1")
                }

                if t.0[k][i] != '.' {
                    println!("{} ({}),{} ({})", i, t.0[k][i], j, t.0[k][j]);
                    panic!("2")
                }

                t.set(k, i, 'O');
                t.set(k, j, '.');
            }

            i = t.cols_len() - 1;
            j = i;
        }
    }

    fn get_weight(t: &Grid) -> i64 {
        let mut total_sum: i64 = 0;
        for (r, row) in t.rows().iter().enumerate() {
            total_sum += row
                .iter()
                .filter_map(|c| {
                    if *c == 'O' {
                        Some(t.rows_len() - r)
                    } else {
                        None
                    }
                })
                .sum::<usize>() as i64;
        }
        total_sum
    }

    fn get_hash(p: &Grid) -> u64 {
        let mut h1 = DefaultHasher::new();

        let str =
            p.0.iter()
                .map(|l| {
                    l.iter()
                        .map(|s| format!("{}", s))
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .collect::<Vec<_>>()
                .join("\n");

        str.hash(&mut h1);

        return h1.finish();
    }

    #[test]
    fn test_part2() {
        let inp = parse_input();

        println!("{}", cycle_n_times(&inp, TOTAL_CYCLES, true));
    }

    fn cycle_n_times(inp: &Grid, total_cycles: i64, allow_mem: bool) -> i64 {
        let mut t: Grid = inp.clone();

        let mut counter = 0;
        let mut mem: HashMap<u64, Vec<i64>> = HashMap::new();

        while counter < total_cycles {
            if !allow_mem {
                t = cycle(t);
                counter += 1;
                continue;
            }
            let hash = get_hash(&t);

            if counter % 21 == 0 {
                println!(
                    "counter:{}, weight:{} ({})",
                    counter,
                    get_weight(&t),
                    mem.contains_key(&hash)
                );
            }

            match mem.get(&hash) {
                Some(prevs) => {
                    let prevs = prevs.clone();
                    let maybe_diff: Option<i64> = prevs
                        .iter()
                        .rev()
                        .map(|prev| 1.max(prev.abs_diff(counter)) as i64)
                        .rev()
                        .find(|diff| counter + diff < total_cycles);

                    // println!(
                    //     "diffs = {:?}",
                    //     prevs
                    //         .iter()
                    //         .rev()
                    //         .map(|prev| 1.max(prev.abs_diff(counter)) as i64)
                    //         .rev()
                    //         .collect::<Vec<_>>()
                    // );

                    if !prevs.contains(&counter) {
                        let mut prevs = prevs.clone();
                        prevs.push(counter);
                        prevs.sort();
                        mem.insert(hash, prevs);
                    }

                    if prevs.len() == 2 {
                        let first = &prevs.get(1).unwrap();
                        let first = **first;
                        let diff: i64 = first.abs_diff(counter) as i64;
                        println!("diff={}, first={}", diff, first);
                        // counter += (TOTAL_CYCLES - counter) % diff;
                        // counter + (Math.floor((total-counter) / diff)*diff)
                        println!("diff: {}", diff);
                        // while counter < TOTAL_CYCLES - diff {
                        //     counter += diff;
                        // }
                        println!("new counter: {}", counter);
                        counter += (total_cycles - counter) - ((total_cycles - counter) % diff);
                        continue;
                    }

                    match maybe_diff {
                        Some(diff) => {
                            counter += diff;
                            println!("new counter: {}", counter)
                        }
                        None => {
                            t = cycle(t);
                            // let mut prevs = prevs.clone();
                            // prevs.push(counter);

                            counter += 1;
                        }
                    }

                    // if counter + key > i {
                    //     t = cycle(t);
                    //     mem.insert(hash, counter);
                    //     counter += 1;
                    // } else {
                    //     counter += key;
                    // }
                }
                None => {
                    // let pre_hash = get_hash(&t);
                    // println!("!Cycle");
                    t = cycle(t);
                    mem.insert(hash, vec![counter]);

                    counter += 1;
                }
            }
        }
        get_weight(&t)
    }

    #[test]
    fn test_single_cycle() {
        let mut inp = parse_input();

        for i in 1..4 {
            println!("{} cycles", i);
            inp = cycle(inp);
            inp.print();
        }
    }

    fn cycle(mut t: Grid) -> Grid {
        for i in 0..4 {
            if i > 0 {
                t = t.rotate_right();
            }

            t = t.rotate_right();
            slide_rocks(&mut t);
            t = t.rotate_left();
        }
        t = t.rotate_right();
        t
    }
}
