#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d15.txt")
            .expect("")
            .replace("\n", "");
        let data = contents.split(",").collect::<Vec<_>>();

        let result = data.iter().map(|l| hash(*l)).sum::<u32>();
        println!("result: {}", result)
    }

    type Hash = String;
    type Lens = u32;

    #[derive(Debug)]
    enum LensOp {
        Remove(Hash),
        Add(Hash, Lens),
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d15.txt")
            .expect("")
            .replace("\n", "");
        let data = contents
            .split(",")
            .map(example_to_lens_op)
            .collect::<Vec<_>>();

        // println!("{:?}", data);

        let mut map: Vec<Vec<(String, Lens)>> = vec![Vec::new(); 256];

        for lo in data {
            // println!("Op: {:?}", lo);
            match lo {
                LensOp::Add(label, lens) => {
                    let h = hash(&label);
                    let mut bbox = map[h as usize].clone();
                    let maybe_existing_lens = bbox.iter().position(|l| l.0 == label);

                    match maybe_existing_lens {
                        Some(pos) => {
                            bbox[pos] = (label, lens);
                        }
                        None => {
                            bbox.push((label, lens));
                        }
                    };

                    map[h as usize] = bbox;
                }
                LensOp::Remove(label) => {
                    let h = hash(&label);
                    let mut bbox = map.get(h as usize).unwrap().clone();
                    match bbox.iter().position(|l| l.0 == label) {
                        Some(pos) => {
                            bbox.remove(pos);
                            map[h as usize] = bbox;
                        }
                        None => {}
                    }
                }
            }
        }

        let mut res = 0;
        for (box_no, m) in map.iter().enumerate() {
            if !m.is_empty() {
                for (pos, (_, lens)) in m.iter().enumerate() {
                    let foc_pow = (1 + box_no) * (pos + 1) * (*lens as usize);

                    // println!("label={}, fp={}", label, foc_pow);
                    res += foc_pow;
                }

                // println!("{:?}", m);
            }
        }
        println!("res: {}", res);
    }

    fn example_to_lens_op(example: &str) -> LensOp {
        let op_type = example.chars().find(|c| *c == '-' || *c == '=').unwrap();

        match op_type {
            '-' => {
                let label = example.replace("-", "");
                LensOp::Remove(label)
            }
            '=' => {
                let (label, lens) = example.split_once("=").unwrap();
                LensOp::Add(label.to_string(), u32::from_str_radix(lens, 10).unwrap())
            }
            c => {
                println!("{}", c);
                unreachable!()
            }
        }
    }

    fn hash(s: &str) -> u32 {
        let mut start: u32 = 0;
        for c in s.chars() {
            if c.is_ascii() {
                start += c as u32;
                // println!("ASCII: {}", start);
                start *= 17;
                start %= 256;
            }
        }
        start
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"))
    }

    #[test]
    fn test_hash2() {
        assert_eq!(30, hash("rn=1"))
    }
}
