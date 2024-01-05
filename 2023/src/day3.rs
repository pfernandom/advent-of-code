#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d3.txt").expect("");

        let mut new_map: Vec<Vec<char>> = contents
            .trim()
            .split("\n")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut locations: Vec<(i32, i32)> = Vec::new();
        let mut digits_map: HashMap<(i32, i32), (i32, i32, i32, i32)> = HashMap::new();
        // let mut all_nums = HashSet::new();
        contents.trim().split("\n").enumerate().for_each(|(i1, l)| {
            let mut row = l.trim().chars().into_iter().enumerate();

            while let Some((mut i2, mut cc)) = row.next() {
                let mut num: Vec<char> = Vec::new();
                let mut num_locs: Vec<(i32, i32)> = Vec::new();
                let (x, mut y) = (i1.try_into().unwrap(), i2.try_into().unwrap());

                let start = (x, y);
                let mut end = (x, y);

                while cc.is_numeric() {
                    num.push(cc);
                    num_locs.push((x, y));
                    let next = row.next();
                    if next.is_none() {
                        end = (x, y + 1);
                        break;
                    }
                    (i2, cc) = next.unwrap();
                    y = i2.try_into().unwrap();
                    end = (x, y);
                }

                if !num.is_empty() {
                    let v =
                        i32::from_str_radix(num.iter().collect::<String>().as_str(), 10).unwrap();
                    for (o1, o2) in num_locs {
                        digits_map.insert((o1, o2), (start.0, start.1, end.1, v));
                        // all_nums.insert(v);
                    }
                }

                if cc != '.' && !cc.is_numeric() {
                    // println!("{}", l);
                    locations.push((x, y));
                }
            }
        });

        let mut dgs = digits_map.values().collect::<Vec<_>>();

        dgs.sort();
        println!("{}, {:?}, digits_map={:?}", contents, locations, dgs);

        // let pre = all_nums.len();

        let mut res = HashSet::new();
        for (i1, i2) in locations {
            let incs: [i32; 3] = [-1, 0, 1];

            for xi in incs {
                for yi in incs {
                    if let Some((start_col, start, end, n)) = digits_map.get(&(xi + i1, yi + i2)) {
                        // result += n;
                        println!(
                            "start={}, end={}, n={}, x={}, y={}",
                            start,
                            end,
                            n,
                            xi + i1,
                            yi + i2
                        );

                        let local_x: usize = (xi + i1).try_into().unwrap();
                        for m in usize::try_from(*start).unwrap()..usize::try_from(*end).unwrap() {
                            new_map[local_x][m] = '_';
                        }

                        res.insert((start_col, start, end, n));
                        // all_nums.remove(n);
                    }
                }
            }
            //    map.get()
        }
        let mut result: i32 = 0;
        for (_start_col, _start, _end, i) in res {
            // println!("{}", i);
            result += i;
        }
        println!("Res: {}", result);
        let print_map = new_map
            .iter()
            .map(|l| {
                return format!("{}\n", l.iter().collect::<String>());
            })
            .collect::<String>();
        println!("{}", print_map);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d3.txt").expect("");

        let mut locations: Vec<(i32, i32)> = Vec::new();
        let mut digits_map: HashMap<(i32, i32), (i32, i32, i32, i32)> = HashMap::new();
        // let mut all_nums = HashSet::new();
        contents.trim().split("\n").enumerate().for_each(|(i1, l)| {
            let mut row = l.trim().chars().into_iter().enumerate();

            while let Some((mut i2, mut cc)) = row.next() {
                let mut num: Vec<char> = Vec::new();
                let mut num_locs: Vec<(i32, i32)> = Vec::new();
                let (x, mut y) = (i1.try_into().unwrap(), i2.try_into().unwrap());

                let start = (x, y);
                let mut end = (x, y);

                while cc.is_numeric() {
                    num.push(cc);
                    num_locs.push((x, y));
                    let next = row.next();
                    if next.is_none() {
                        end = (x, y + 1);
                        break;
                    }
                    (i2, cc) = next.unwrap();
                    y = i2.try_into().unwrap();
                    end = (x, y);
                }

                if !num.is_empty() {
                    let v =
                        i32::from_str_radix(num.iter().collect::<String>().as_str(), 10).unwrap();
                    for (o1, o2) in num_locs {
                        digits_map.insert((o1, o2), (start.0, start.1, end.1, v));
                        // all_nums.insert(v);
                    }
                }

                if cc == '*' {
                    // println!("{}", l);
                    locations.push((x, y));
                }
            }
        });

        let mut dgs = digits_map.values().collect::<Vec<_>>();

        dgs.sort();
        println!("{}, {:?}, digits_map={:?}", contents, locations, dgs);

        // let pre = all_nums.len();

        let mut res = HashSet::new();
        for (i1, i2) in locations {
            let incs: [i32; 3] = [-1, 0, 1];

            // let count = 0;
            let mut sub_res = HashSet::new();

            for xi in incs {
                for yi in incs {
                    if let Some((start_col, start, end, n)) = digits_map.get(&(xi + i1, yi + i2)) {
                        // result += n;
                        println!(
                            "start={}, end={}, n={}, x={}, y={}",
                            start,
                            end,
                            n,
                            xi + i1,
                            yi + i2
                        );

                        // count += 1;

                        // let local_x: usize = (xi + i1).try_into().unwrap();
                        // for m in usize::try_from(*start).unwrap()..usize::try_from(*end).unwrap() {
                        //     new_map[local_x][m] = '_';
                        // }

                        sub_res.insert((start_col, start, end, n));
                        // all_nums.remove(n);
                    }
                }
            }
            if sub_res.len() == 2 {
                let mut mul = 1;
                let mut sc = 1;
                let mut ss = 1;
                let mut se = 1;

                for (col, start, end, i) in sub_res {
                    mul *= i;
                    sc += col;
                    ss += start;
                    se += end;
                }
                res.insert((sc, ss, se, mul));
            }
            //    map.get()
        }
        let mut result: i32 = 0;
        for (_start_col, _start, _end, i) in res {
            // println!("{}", i);
            result += i;
        }
        println!("Res: {}", result);
        // let print_map = new_map
        //     .iter()
        //     .map(|l| {
        //         return format!("{}\n", l.iter().collect::<String>());
        //     })
        //     .collect::<String>();
        // println!("{}", print_map);
    }
}
