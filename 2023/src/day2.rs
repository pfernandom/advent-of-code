#[cfg(test)]
mod tests {

    use crate::read_file;

    fn to_color(v: &str) -> [u32; 3] {
        let (count_str, color) = v.trim().split_once(" ").unwrap();
        let count = count_str.parse().unwrap();
        match color {
            "red" => [count, 0, 0],
            "green" => [0, count, 0],
            "blue" => [0, 0, count],
            _ => panic!("Unknown color"),
        }
    }

    fn max(a: u32, b: u32) -> u32 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn p1_line(str: String) -> [u32; 3] {
        let (_, n): (&str, &str) = str.split_once(":").unwrap();

        let sets: [u32; 3] = n
            .split(";")
            .map(|s| {
                s.trim()
                    .split(',')
                    .map(|c| to_color(c))
                    .fold([0, 0, 0], |a, c| {
                        [max(a[0], c[0]), max(a[1], c[1]), max(a[2], c[2])]
                    })
            })
            .fold([0, 0, 0], |a, c| {
                [max(a[0], c[0]), max(a[1], c[1]), max(a[2], c[2])]
            });

        return sets;
    }

    #[test]
    fn p1() {
        let mut i = 1;
        let mut res = 0;
        for line in read_file::read_file("./problems/d2.txt".to_string()) {
            let locations = p1_line(line);

            println!(
                "{}",
                format!(
                    "R={}, G={}, B={}\n",
                    locations[0], locations[1], locations[2]
                )
            );

            if locations[0] <= 12 && locations[1] <= 13 && locations[2] <= 14 {
                res += i;
            }
            i += 1;
        }
        println!("Res: {}", res);
    }

    #[test]
    fn p2() {
        let mut res = 0;
        for line in read_file::read_file("./problems/d2.txt".to_string()) {
            let locations = p1_line(line);

            println!(
                "{}",
                format!(
                    "R={}, G={}, B={}\n",
                    locations[0], locations[1], locations[2]
                )
            );

            let power = locations[0] * locations[1] * locations[2];
            println!("Power: {}", power);
            res += power;
        }
        println!("Res: {}", res);
    }
}
