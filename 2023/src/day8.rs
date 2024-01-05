#![allow(dead_code)]

use num_integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use tokio_stream::StreamExt; // Trait for primitive integer types

fn lcm_list(list: Vec<i64>) -> i64 // Restrict T to types that are primitive integers
{
    list.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}

type Map = HashMap<String, (String, String)>;

fn traverse(map: &Map, start: &String, dir: &mut Direction) -> i64 {
    let mut cur = start;
    let mut count = 0;

    while cur != "ZZZ" {
        // println!("cur={}", cur);
        let (left, right) = map.get(cur).expect("Get key");
        cur = match dir.next() {
            'L' => left,
            'R' => right,
            _ => panic!("unexpected direction"),
        };
        count += 1;
    }

    return count;
}

async fn traverse2(map: &Map, starts: Vec<&String>, dir: &mut Direction) -> i64 {
    static END_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Z$").unwrap());
    let mut cur: Vec<String> = starts.iter().map(|c| c.to_string()).collect::<Vec<_>>();
    let mut count = 0;

    // println!("{:?}", cur);

    while !tokio_stream::iter(cur.iter())
        .all(|f| END_RE.is_match(f))
        .await
    {
        let next = dir.next();
        // println!("cur={}", cur);

        cur = tokio_stream::iter(cur.iter())
            .map(|c| {
                let (left, right) = map.get(c).expect("Get key");
                match next {
                    'L' => left.clone(),
                    'R' => right.clone(),
                    _ => panic!("unexpected direction"),
                }
            })
            .collect::<Vec<_>>()
            .await;

        count += 1;

        // println!("{}, {:?}", next, cur);
        if count % 100001 == 0 {
            println!("count={}", count);
        }
    }

    return count;
}

fn traverse3(map: &Map, start: &String, dir: &mut Direction) -> i64 {
    let mut cur = start;
    let mut count = 0;
    static END_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Z$").unwrap());

    while !END_RE.is_match(&cur) {
        // println!("cur={}", cur);
        let (left, right) = map.get(cur).expect("Get key");
        cur = match dir.next() {
            'L' => left,
            'R' => right,
            _ => panic!("unexpected direction"),
        };
        count += 1;
    }

    return count;
}

struct Direction {
    pat: Vec<char>,
    index: usize,
}

impl Direction {
    fn new(pat: String) -> Self {
        let pat = pat.chars().collect::<Vec<_>>();
        Self { index: 0, pat }
    }
    fn next(&mut self) -> char {
        let n = self.pat[self.index];
        if self.index + 1 == self.pat.len() {
            self.index = 0;
        } else {
            self.index += 1;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use once_cell::sync::Lazy;
    use regex::Regex;

    use crate::day8::{traverse, traverse2, traverse3, Direction};

    use super::lcm_list;

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d8.txt").expect("");

        let mut it = contents.split("\n").into_iter();

        let first = it.next().unwrap();

        let x = it
            .map(|l| l.trim().split_once("=").expect("Equals"))
            .map(|(from, r)| (from, r.trim().split_once(",").unwrap()))
            .map(|(from, (t1, t2))| {
                (
                    from.trim().to_string(),
                    (
                        t1.replace("(", "").trim().to_string(),
                        t2.replace(")", "").trim().to_string(),
                    ),
                )
            })
            .collect::<HashMap<String, (String, String)>>();

        let mut dir = Direction::new(first.to_string());
        let start = "AAA".to_string();

        println!("first={}, {:?}", first, x);

        let result = traverse(&x, &start, &mut dir);

        println!("result: {}", result)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 6)]
    async fn p2() {
        let contents = fs::read_to_string("./problems/d8.txt").expect("");

        let mut it = contents.split("\n").into_iter();

        static START_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"A$").unwrap());
        static END_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Z$").unwrap());

        let first = it.next().unwrap();

        let x = it
            .map(|l| l.trim().split_once("=").expect("Equals"))
            .map(|(from, r)| (from, r.trim().split_once(",").unwrap()))
            .map(|(from, (t1, t2))| {
                (
                    from.trim().to_string(),
                    (
                        t1.replace("(", "").trim().to_string(),
                        t2.replace(")", "").trim().to_string(),
                    ),
                )
            })
            .collect::<HashMap<String, (String, String)>>();

        let mut dir = Direction::new(first.to_string());

        let starts: Vec<&String> = x
            .iter()
            .filter_map(|(start, _)| {
                if START_RE.is_match(start) {
                    Some(start)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("starts={}", starts.len());

        let result = traverse2(&x, starts, &mut dir).await;

        println!("result: {}", result)
    }

    #[test]
    fn p2_alt() {
        let contents = fs::read_to_string("./problems/d8.txt").expect("");
        let mut it = contents.split("\n").into_iter();

        let first = it.next().unwrap();

        let x = it
            .map(|l| l.trim().split_once("=").expect("Equals"))
            .map(|(from, r)| (from, r.trim().split_once(",").unwrap()))
            .map(|(from, (t1, t2))| {
                (
                    from.trim().to_string(),
                    (
                        t1.replace("(", "").trim().to_string(),
                        t2.replace(")", "").trim().to_string(),
                    ),
                )
            })
            .collect::<HashMap<String, (String, String)>>();

        let mut dir = Direction::new(first.to_string());

        // println!("first={}, {:?}", first, x);

        static START_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"A$").unwrap());

        let starts: Vec<&String> = x
            .iter()
            .filter_map(|(start, _)| {
                if START_RE.is_match(start) {
                    Some(start)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("starts={}", starts.len());

        let mut results = Vec::new();
        for s in starts {
            let result = traverse3(&x, &s, &mut dir);
            results.push(result);
        }

        println!("result: {}", lcm_list(results))
    }

    // #[test]
    // fn test_regex() {
    //     static START_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"A$").unwrap());
    //     static END_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Z$").unwrap());

    //     println!("{}", END_RE.is_match("ZBZ"));
    // }
}
