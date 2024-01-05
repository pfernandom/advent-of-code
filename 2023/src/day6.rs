#![allow(dead_code)]

use std::fs;

use once_cell::sync::Lazy;
use regex::Regex;

fn parse_input() -> Vec<(i64, i64)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

    let contents = fs::read_to_string("./problems/d6.txt").expect("");
    let data: Vec<Vec<_>> = contents
        .split("\n")
        .map(|l| {
            l.trim()
                .split_whitespace()
                .skip(1)
                .map(|col| col.trim())
                .map(|col| col.parse().expect("Could not parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    std::iter::zip(&data[0], &data[1])
        .map(|pair| (*pair.0, *pair.1))
        .collect::<Vec<_>>()
}

fn parse_input2() -> (i64, i64) {
    let contents = fs::read_to_string("./problems/d6.txt").expect("");
    let data: Vec<_> = contents
        .split("\n")
        .map(|l| {
            l.trim()
                .split_whitespace()
                .skip(1)
                .map(|col| col.trim())
                .flat_map(|col| col.chars())
                .collect::<String>()
        })
        .map(|r| r.parse().expect("Should parse number"))
        .collect::<Vec<_>>();

    (data[0], data[1])
}

fn find_hold_time(race_time: i64, record_dist: i64) -> i64 {
    // let mut max = 0;
    let mut max_charge = 0;

    for t in 0..race_time {
        let rest_time = race_time - t;
        let speed = t;

        let travel_dist = speed * rest_time;

        if travel_dist > record_dist {
            max_charge += 1;
        }
    }

    println!("max_charge={}", max_charge);
    return max_charge;
}

#[cfg(test)]
mod tests {
    use std::{fs, time::Instant};
    use tokio_stream::StreamExt;

    use once_cell::sync::Lazy;
    use regex::Regex;

    use crate::day6::{find_hold_time, parse_input, parse_input2};

    #[test]
    fn p1() {
        let races = parse_input();

        let mut result = 1;
        for (time, record_dist) in &races {
            result *= find_hold_time(*time, *record_dist);
        }

        println!("{:?}", result);
    }

    #[test]
    fn p2() {
        let (time, record_dist) = parse_input2();
        let result = find_hold_time(time, record_dist);

        println!("{:?}", result);
    }

    #[test]
    fn parse() {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
        let contents = fs::read_to_string("./problems/d6.txt").expect("");
        let data: Vec<Vec<i64>> = contents
            .split("\n")
            .map(|str| {
                RE.captures_iter(str)
                    .map(|c| c[0].to_string())
                    .map(|c| c.parse().expect("Should parse number"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // let mut iter = RE.captures_iter(contents.as_str());

        // while let Some(cap) = iter.next() {
        //     println!("{}", &cap[0]);
        // }

        println!("{:?}", data);
    }

    fn parse_sync(contents: &String, re: &Regex) -> Vec<Vec<i64>> {
        contents
            .split("\n")
            .map(|str| {
                re.captures_iter(str)
                    .map(|c| c[0].to_string())
                    .map(|c| c.parse().expect("Should parse number"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    async fn parse_async(contents: &String, re: &Regex) -> Vec<Vec<i64>> {
        let stream = tokio_stream::iter(contents.split("\n"))
            .map(|str| {
                re.captures_iter(str)
                    .map(|c| c[0].to_string())
                    .map(|c| c.parse().expect("Should parse number"))
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<_>>>();
        stream.await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn async_test() {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
        let contents = fs::read_to_string("./problems/d6.txt").expect("");

        let mut now = Instant::now();

        for _ in 0..2000 {
            let _d1 = parse_sync(&contents, &RE);
        }

        let mut elapsed = now.elapsed();

        println!("Elapsed (sync): {:.2?}", elapsed);

        now = Instant::now();

        for _ in 0..2000 {
            let _d2 = parse_async(&contents, &RE).await;
        }

        elapsed = now.elapsed();

        println!("Elapsed (async): {:.2?}", elapsed);
    }
}
