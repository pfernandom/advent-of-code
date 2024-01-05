#![allow(dead_code)]

static LOG: bool = false;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::day12::LOG;

    fn max(d1: i64, d2: i64) -> i64 {
        d1.max(d2)
    }

    fn max3(d1: i64, d2: i64, d3: i64) -> i64 {
        d1.max(d2).max(d3)
    }

    type Groups = i64;
    type Index = usize;

    fn solve(
        left: &Vec<char>,
        groups: Vec<i64>,
        i: Index,
        cache: &mut HashMap<(String, Index), i64>,
    ) -> i64 {
        let sergroups = groups.iter().map(|c| format!("{}", c)).collect::<String>();

        if LOG {
            println!("{} {}", left.iter().skip(i).collect::<String>(), sergroups);
        }
        if groups.len() == 0 {
            let remaining = left.iter().skip(i).find(|c| **c == '#').is_some();
            if remaining || left.len() < i {
                return 0;
            } else {
                return 1;
            }
        }

        let mut next = match left.get(i) {
            Some(el) => el,
            None => {
                cache.insert((sergroups.clone(), i), 0);
                return 0;
            }
        };
        let mut i = i;

        while *next != '?' && *next != '#' {
            i += 1;
            next = match left.get(i) {
                Some(el) => el,
                None => {
                    cache.insert((sergroups.clone(), i), 0);
                    return 0;
                }
            }
        }

        if cache.contains_key(&(sergroups.clone(), i)) {
            return *cache.get(&(sergroups.clone(), i)).unwrap();
        }

        let group = *groups.get(0).unwrap();

        if *next == '#' {
            let direct_matches = left.iter().skip(i).take_while(|c| **c == '#').count();
            if (direct_matches as i64) > group {
                if LOG {
                    println!("Not enough direct matches");
                }
                cache.insert((sergroups.clone(), i), 0);
                return 0;
            }
        }

        let semi_direct_matches = left
            .iter()
            .skip(i)
            .take_while(|c| **c == '#' || **c == '?')
            .collect::<Vec<_>>();
        if (semi_direct_matches.len() as i64) < group {
            if semi_direct_matches.contains(&&'#') {
                return 0;
            }
            let res = solve(left, groups, i + semi_direct_matches.len() + 1, cache);
            cache.insert((sergroups.clone(), i), res);
            return res;
        }

        if *next == '?' {}

        let next_is_pnd = left
            .iter()
            .skip(i)
            .skip(group as usize)
            .next()
            .map(|c| *c == '#')
            .unwrap_or(false);

        if next_is_pnd && LOG {
            println!("next_is_pnd")
        }

        let pos_next_inc = i + (group as usize) + 1;

        let cont = if next_is_pnd {
            0
        } else {
            if LOG {
                println!("pos_next_inc={}", pos_next_inc);
            }
            solve(
                left,
                groups[1..].to_vec(),
                left.len().min(pos_next_inc),
                cache,
            )
        };

        let skip = if *next == '?' {
            solve(left, groups, i + 1, cache)
        } else {
            0
        };
        let res = cont + skip;

        cache.insert((sergroups.clone(), i), res);
        if LOG {
            let slice = left.iter().skip(i).collect::<String>();
            println!(
                "left:{}, groups:{}, cont={}, skip={}, res={}",
                slice, sergroups, cont, skip, res
            );
        }
        return res;
    }

    #[test]
    fn test_solve() {
        let reslt = solve(&vec!['?', '?'], vec![1], 0, &mut HashMap::new());
        assert_eq!(reslt, 2, "??");

        let reslt = solve(&vec!['?', '#'], vec![1], 0, &mut HashMap::new());
        assert_eq!(reslt, 1, "?#");

        let reslt = solve(&vec!['#', '?'], vec![1], 0, &mut HashMap::new());
        assert_eq!(reslt, 1, "#?");

        let reslt = solve(&vec!['?', '?', '?'], vec![1, 1], 0, &mut HashMap::new());
        assert_eq!(reslt, 1, "???");

        let reslt = solve(&vec!['?', '#', '.'], vec![2], 0, &mut HashMap::new());
        assert_eq!(reslt, 1, "?#.");

        let reslt = solve(&vec!['?', '#', '?', '.'], vec![2], 0, &mut HashMap::new());
        assert_eq!(reslt, 2, "?#?.");

        let reslt = solve(
            &vec!['?', '?', '.', '?', '?', '.', '#', '#', '#'],
            vec![1, 1, 3],
            0,
            &mut HashMap::new(),
        );
        assert_eq!(reslt, 4);
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d12.txt").expect("");
        let g: Vec<(&str, &str)> = contents
            .split("\n")
            .filter_map(|l| l.split_once(" "))
            .collect::<Vec<_>>();

        let mut res = 0;
        for (left, right) in g {
            let subres = solve(
                &left.chars().collect::<Vec<_>>(),
                right
                    .split(",")
                    .map(|c| i64::from_str_radix(c, 10).unwrap())
                    .collect::<Vec<_>>(),
                0,
                &mut HashMap::new(),
            );
            res += subres;
            println!("{}, {}: {}", left, right, subres);
        }

        println!("\nres: {}", res);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d12.txt").expect("");
        let g: Vec<(&str, &str)> = contents
            .split("\n")
            .filter_map(|l| l.split_once(" "))
            .collect::<Vec<_>>();

        let mut res = 0;

        for (left, right) in g {
            let left = vec![left; 5].join("?");
            let right = vec![right; 5].join(",");
            let subres = solve(
                &left.chars().collect::<Vec<_>>(),
                right
                    .split(",")
                    .map(|c| i64::from_str_radix(c, 10).unwrap())
                    .collect::<Vec<_>>(),
                0,
                &mut HashMap::new(),
            );
            res += subres;
            println!("{}, {}: {}", left, right, subres);
        }

        println!("\nres: {}", res);
    }
}
