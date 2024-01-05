#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::fs;

    fn find_pattern(line: Vec<i64>) -> (Vec<i64>, i64, i64, i64) {
        let mut res = Vec::new();
        let mut prev = 0;
        let mut zeros = 0;
        let mut last = 0;
        let mut first = 0;
        for (i, n) in line.iter().enumerate() {
            if i == 0 {
                prev = *n;
                first = *n;
            } else {
                res.push(n - prev);
                zeros += if (n - prev) == 0 { 1 } else { 0 };
                prev = *n;
            }
            last = *n;
        }
        (
            res,
            if zeros == line.len() - 1 { 0 } else { 1 },
            last,
            first,
        )
    }

    fn solve(nums: Vec<i64>) -> (i64, i64) {
        println!("{:?}", nums);

        let (mut p, mut sum, mut last, mut first) = find_pattern(nums);

        let mut spaces = 1;
        let mut sp = vec!["  "; spaces].join(" ");
        println!("{}{:?}", sp, p);

        let mut lasts: Vec<_> = Vec::new();
        let mut firsts: Vec<_> = Vec::new();
        lasts.push(last);
        firsts.push(first);
        // let mut last_p = p;

        while sum != 0 {
            (p, sum, last, first) = find_pattern(p);
            spaces += 1;
            sp = vec!["  "; spaces].join(" ");
            println!("{}{:?}", sp, p);
            lasts.push(last);
            firsts.push(first);
        }

        let mut sum_firsts = 0;
        while let Some(n) = firsts.pop() {
            sum_firsts = n - sum_firsts;
        }

        let mut sum_lasts = 0;
        while let Some(n) = lasts.pop() {
            sum_lasts += n;
        }

        (sum_firsts, sum_lasts)
    }

    #[test]
    fn p1_and_2() {
        let contents = fs::read_to_string("./problems/d9.txt").expect("");

        let data: Vec<Vec<i64>> = contents
            .split("\n")
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        println!("{:?}", data);

        let (mut sum_firsts, mut sum_lasts) = (0, 0);
        for row in data {
            println!("== Row ===");
            let (first, last) = solve(row.clone());
            sum_firsts += first;
            sum_lasts += last;
        }

        println!("P1: {}, P2: {}", sum_lasts, sum_firsts);
    }
}
