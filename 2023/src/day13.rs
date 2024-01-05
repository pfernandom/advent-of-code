#![allow(dead_code)]

#[cfg(test)]
mod tests {

    use std::{fs, iter::zip, usize};

    type Problem = Vec<Vec<char>>;

    fn parse_input() -> Vec<Problem> {
        let contents = fs::read_to_string("./problems/d13.txt").expect("");

        let mut problems = Vec::new();

        let mut subp = Vec::new();

        for line in contents.lines() {
            if line.trim().is_empty() {
                problems.push(subp);
                subp = Vec::new();
            } else {
                subp.push(line.chars().collect::<Vec<_>>())
            }
        }

        if !subp.is_empty() {
            problems.push(subp);
        }

        problems
    }

    fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let height = grid.len();
        let width = grid[0].len();

        let mut v: Vec<Vec<char>> = Vec::new();

        for w in 0..width {
            let mut row: Vec<_> = Vec::new();
            for h in 0..height {
                // println!("{:?}")
                row.insert(0, grid[h][w]);
            }
            v.push(row)
        }

        v
    }

    fn print(p: &Problem) {
        println!();
        for l in p {
            let s = l.iter().collect::<String>();
            println!("{}", s);
        }
    }

    fn count_diffs(cols: usize, rows: usize, p: &Problem) -> usize {
        // for line in p {
        let line = &p[rows];
        let line_and_ind = line.iter().enumerate().collect::<Vec<_>>();
        let (left, right) = line_and_ind.split_at(cols);

        let diffs = zip(left.iter().rev(), right)
            .map(|(lc, rc)| lc.1 == rc.1)
            .filter(|e| *e == false)
            .collect::<Vec<_>>();

        let diff = diffs.len();
        diff
    }

    #[test]
    fn p1_and_p2() {
        let inp = parse_input();

        for part in [0, 1] {
            let mut total_res = 0;
            for p in &inp {
                let mut hsplits = 0;
                let mut wsplits = 0;

                p.get(0).map(|fline| {
                    for i in 1..fline.len() {
                        let mut sum_diff = 0;
                        for j in 0..p.len() {
                            sum_diff += count_diffs(i, j, &p);
                        }

                        if sum_diff == part {
                            hsplits = i;
                        }
                    }
                });

                let pt = rotate(&p);

                pt.get(0).map(|fline| {
                    for i in 1..fline.len() {
                        let mut sum_diff = 0;
                        for j in 0..pt.len() {
                            sum_diff += count_diffs(i, j, &pt);

                            // println!("diff: {}", sum_diff);
                        }

                        if sum_diff == part {
                            wsplits += fline.len() - i;
                        }
                    }
                });

                total_res += hsplits + (100 * wsplits);
            }
            println!("res: {}", total_res)
        }
    }
}
