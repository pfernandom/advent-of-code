#![allow(dead_code)]
use std::cmp::Ordering;

#[derive(Debug)]
struct RangeExt {
    start: i32,
    end: i32,
    inclusive_end: bool,
}

impl RangeExt {
    fn new(start: i32, end: i32) -> Self {
        assert!(start < end);
        Self {
            start,
            end,
            inclusive_end: false,
        }
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.end <= other.start || self.start >= other.end {
            return None;
        }
        Some(Self::new(
            self.start.max(other.start),
            self.end.min(other.end),
        ))
    }

    fn contains(&self, other: &Self) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn ldiff(&self, other: &Self) -> Option<Self> {
        if self.start > other.start {
            return None;
        }
        Some(Self::new(self.start, other.start))
    }

    fn rdiff(&self, other: &Self) -> Option<Self> {
        if self.start < other.start {
            return None;
        }
        Some(Self::new(other.end, self.end))
    }

    fn partition(&self, other: &Self) -> (Option<RangeExt>, Option<RangeExt>, Option<RangeExt>) {
        (
            self.ldiff(other),
            self.intersection(other),
            self.rdiff(other),
        )
    }
}

#[cfg(test)]
mod range_tests {
    use super::RangeExt;

    #[test]
    fn test_intersection() {
        let r1 = RangeExt::new(5, 15);
        let r2 = RangeExt::new(0, 10);

        let maybe_diff = r1.intersection(&r2);
        assert!(maybe_diff.is_some());
        let diff = maybe_diff.unwrap();

        println!("start={}, end={}", diff.start, diff.end)
    }

    #[test]
    fn test_ldiff() {
        let r1 = RangeExt::new(5, 15);
        let r2 = RangeExt::new(0, 10);

        let maybe_diff = r2.ldiff(&r1);
        assert!(maybe_diff.is_some());
        let diff = maybe_diff.unwrap();
        println!("start={}, end={}", diff.start, diff.end)
    }

    #[test]
    fn test_rdiff() {
        let r1 = RangeExt::new(5, 15);
        let r2 = RangeExt::new(0, 10);

        let maybe_diff = r1.rdiff(&r2);
        assert!(maybe_diff.is_some());
        let diff = maybe_diff.unwrap();
        println!("start={}, end={}", diff.start, diff.end)
    }

    #[test]
    fn test_partition() {
        let r1 = RangeExt::new(5, 15);
        let r2 = RangeExt::new(0, 10);

        let p1 = r1.partition(&r2);
        let p2 = r2.partition(&r1);
        println!("{:?}", p1);
        println!("{:?}", p2);

        assert!(p1.0.is_none());
        assert!(p1.1.is_some());
        assert!(p1.2.is_some());

        assert!(p2.0.is_some());
        assert!(p2.1.is_some());
        assert!(p2.2.is_none());
    }

    #[test]
    fn test_partition2() {
        let r1 = RangeExt::new(5, 10);
        let r2 = RangeExt::new(10, 15);

        let p1 = r1.partition(&r2);

        assert!(p1.0.is_some());
        assert!(p1.1.is_none());
        assert!(p1.2.is_none());

        let p1 = r2.partition(&r1);

        assert!(p1.0.is_none());
        assert!(p1.1.is_none());
        assert!(p1.2.is_some());
    }
}

#[derive(Eq)]
pub struct MapDescription {
    dest_start: usize,
    source_start: usize,
    range_len: usize,
}

impl PartialOrd for MapDescription {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapDescription {
    fn eq(&self, other: &Self) -> bool {
        self.source_start == other.source_start
    }
}

impl Ord for MapDescription {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_start.cmp(&other.source_start)
    }
}

#[derive(Debug)]
enum RangeType {
    Same,
    SameR,
    Range((usize, usize)),
    Range2((usize, usize), (usize, usize)),
    Range2R((usize, usize), (usize, usize)),
    Range3((usize, usize), (usize, usize), (usize, usize)),
}

impl MapDescription {
    fn new(dest_start: usize, source_start: usize, range_len: usize) -> Self {
        Self {
            dest_start,
            source_start,
            range_len,
        }
    }

    fn map_src_to_dest_range(&self, rstart: usize, rlen: usize) -> RangeType {
        let rend = rstart + rlen;
        let selfend = self.source_start + self.range_len;

        // case 1: s1 e1 [s2 e2]
        if rend < self.source_start {
            return RangeType::Same;
        }

        // case 2: [s2 e2]  s1 e1
        if rstart >= selfend {
            return RangeType::SameR;
        }

        // case 3: s1 [s2 e1 e2]
        if rstart < self.source_start && rend <= selfend {
            return RangeType::Range2(
                (rstart, self.source_start - rstart),
                (
                    self.dest_start,
                    if rend == self.source_start {
                        1
                    } else {
                        rend - self.source_start
                    },
                ),
            );
        }

        // case 4:  [s2 s1 e1 e2]
        if rstart >= self.source_start && rend <= selfend {
            let s1_len = rstart - self.source_start;

            let s1 = self.dest_start + s1_len;
            let e1 = rlen;

            return RangeType::Range((s1, e1));
        }

        // case 5:  [s2 s1 e2] e1
        if rstart >= self.source_start && rstart < selfend && rend > selfend {
            let s1_len = rstart - self.source_start;

            let s1 = self.dest_start + s1_len;
            let e1 = if self.range_len == s1_len {
                1
            } else {
                self.range_len - s1_len
            };

            return RangeType::Range2R((s1, e1), (self.source_start + self.range_len, rlen - e1));
        }

        // case 6: s1 [s2 e2] e1
        if rstart < self.source_start && rend > selfend {
            let l1 = self.source_start - rstart;
            let l2 = self.range_len;
            let l3 = rlen - (l2 + l1);
            return RangeType::Range3(
                (rstart, l1),
                (self.dest_start, l2),
                (rstart + l2 + l1 + 1, l3),
            );
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests_map_descr {
    use super::MapDescription;

    #[test]
    fn case1() {
        let map = MapDescription::new(52, 50, 48);
        match map.map_src_to_dest_range(10, 5) {
            super::RangeType::Same => {}
            _ => panic!(""),
        }
    }

    #[test]
    fn case2() {
        let map = MapDescription::new(52, 50, 2);
        match map.map_src_to_dest_range(54, 5) {
            super::RangeType::SameR => {}
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case3() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(49, 3) {
            super::RangeType::Range2(r1, r2) => {
                assert_eq!(r1.0, 49);
                assert_eq!(r1.1, 1);

                assert_eq!(r2.0, 100);
                assert_eq!(r2.1, 2);
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case3_extra1() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(45, 5) {
            super::RangeType::Range2(r1, r2) => {
                assert_eq!(r1.0, 45);
                assert_eq!(r1.1, 5);

                assert_eq!(r2.0, 100);
                assert_eq!(r2.1, 1);
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case4() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(51, 3) {
            super::RangeType::Range(r) => {
                assert_eq!(r.0, 101);
                assert_eq!(r.1, 3);
            }
            _ => panic!(""),
        }
    }

    #[test]
    fn case4_e1() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(50, 5) {
            super::RangeType::Range(r) => {
                assert_eq!(r.0, 100);
                assert_eq!(r.1, 5);
            }
            _ => panic!(""),
        }
    }

    #[test]
    fn case5() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(51, 6) {
            super::RangeType::Range2R(r1, r2) => {
                assert_eq!(r1.0, 101);
                assert_eq!(r1.1, 4);

                assert_eq!(r2.0, 55);
                assert_eq!(r2.1, 2);
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case5_e1() {
        let map = MapDescription::new(100, 50, 6);
        match map.map_src_to_dest_range(55, 6) {
            super::RangeType::Range2R(r1, r2) => {
                assert_eq!(r1.0, 105);
                assert_eq!(r1.1, 1);

                assert_eq!(r2.0, 56);
                assert_eq!(r2.1, 5);
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case5_e2() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(55, 6) {
            super::RangeType::SameR => {}
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case6() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(49, 7) {
            super::RangeType::Range3(r1, r2, r3) => {
                assert_eq!(r1.0, 49);
                assert_eq!(r1.1, 1);

                assert_eq!(r2.0, 100);
                assert_eq!(r2.1, 5);

                assert_eq!(r3.0, 56);
                assert_eq!(r3.1, 1);
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn case_extra1() {
        let map = MapDescription::new(100, 50, 5);
        match map.map_src_to_dest_range(50, 7) {
            super::RangeType::Range2R(r1, r2) => {
                assert_eq!(r1.0, 100);
                assert_eq!(r1.1, 5);

                assert_eq!(r2.0, 55);
                assert_eq!(r2.1, 2);
            }
            r => panic!("{:?}", r),
        }
    }

    // #[test]
    // fn case_extra2() {
    //     let map = MapDescription::new(49, 53, 8);
    //     match map.map_src_to_dest_range(57, 1) {
    //         r => panic!("{:?}", r),
    //     }
    // }

    //49 53 8
}

impl std::fmt::Display for MapDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.dest_start, self.source_start, self.range_len
        )
    }
}

struct Map {
    name: String,
    maps: Vec<MapDescription>,
}

impl Map {
    fn new(name: String) -> Self {
        Self {
            name,
            maps: Vec::new(),
        }
    }

    fn empty() -> Self {
        Self {
            name: String::from(""),
            maps: Vec::new(),
        }
    }

    fn get_dest(&self, source: usize) -> usize {
        let found = self.maps.iter().find(|m| {
            if source < m.source_start || source > m.source_start + m.range_len {
                return false;
            }
            return true;
        });

        match found {
            Some(m) => m.dest_start + (source - m.source_start),
            None => source,
        }
    }

    fn get_des_by_range(&self, range_start: usize, range_len: usize) -> Vec<(usize, usize)> {
        let mut cur_range_start = range_start;
        let mut cur_range_len = range_len;

        let mut results: Vec<(usize, usize)> = Vec::new();
        let mut higher = false;

        for m in &self.maps {
            match m.map_src_to_dest_range(cur_range_start, cur_range_len) {
                RangeType::Same => {
                    results.push((cur_range_start, cur_range_len));
                    break;
                }
                RangeType::SameR => {
                    higher = true;
                    continue;
                }
                RangeType::Range(r) => {
                    higher = false;
                    results.push(r);
                    break;
                }
                RangeType::Range2(r1, r2) => {
                    higher = false;
                    results.push(r1);
                    results.push(r2);
                    break;
                }
                RangeType::Range2R(r1, r2) => {
                    higher = true;
                    results.push(r1);

                    cur_range_start = r2.0;
                    cur_range_len = r2.1;
                }
                RangeType::Range3(r1, r2, r3) => {
                    higher = true;
                    results.push(r1);
                    results.push(r2);

                    cur_range_start = r3.0;
                    cur_range_len = r3.1;
                }
            }
        }

        if higher {
            results.push((cur_range_start, cur_range_len))
        }
        results
    }
}

impl<'a> std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "| {} \n", self.name).expect("fmt");

        for m in &self.maps {
            write!(f, " - {} \n", m).expect("fmt");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test2 {
    use std::fs;

    use crate::day5::Input;

    use super::{Map, MapDescription};

    #[test]
    fn test1() {
        let p: (usize, usize) = (68, 7);
        let mut map = Map::new("temperature-to-humidity".to_string());
        // temperature-to-humidity map:
        // 0 69 1
        // 1 0 69

        map.maps.push(MapDescription::new(1, 0, 69));
        map.maps.push(MapDescription::new(0, 69, 1));

        let ranges = map.get_des_by_range(p.0, p.1);

        println!("ranges={:?}", ranges);
    }

    #[test]
    fn test2_pre() {
        let p: (usize, usize) = (93, 1);
        let mut map = Map::new("temperature-to-humidity".to_string());
        // temperature-to-humidity map:
        // 0 69 1
        // 1 0 69

        map.maps.push(MapDescription::new(60, 56, 37));
        map.maps.push(MapDescription::new(56, 93, 4));
        // map.maps.push(MapDescription::new(0, 69, 1));

        let ranges = map.get_des_by_range(p.0, p.1);

        println!("ranges={:?}", ranges);
    }

    #[test]
    fn test2() {
        let contents = fs::read_to_string("./problems/d5_test.txt").expect("");
        let inp = Input::parse(contents);
        let mut ranges: Vec<(usize, usize)> = inp.seeds_to_ranges();
        // let mut ranges: Vec<(usize, usize)> = vec![(55, 13)];
        for map in &inp.maps {
            println!("Look into map: {}", map);

            let mut map_ranges: Vec<(usize, usize)> = Vec::new();

            for ran in ranges {
                let expected = map.get_dest(ran.0);

                println!("Expected: {} for start {}", expected, ran.0);

                let mut rs = map.get_des_by_range(ran.0, 1);

                println!("rs={:?}", rs);

                let found = rs.iter().find(|r| r.0 == expected);

                assert!(found.is_some());

                map_ranges.append(&mut rs);
            }

            ranges = map_ranges;
            println!("New ranges: {:?}", ranges);
        }

        let mut m = usize::MAX;

        for r in &ranges {
            m = m.min(r.0);
        }

        println!("=Ranges: {:?}", ranges);
        println!("=Min: {}", m);
    }
}

pub struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seeds = self
            .seeds
            .iter()
            .map(|n| format!("{}, ", n))
            .collect::<String>();
        write!(f, "Seeds: {} \n", seeds).expect("fmt");

        for m in &self.maps {
            write!(f, "- Map: {} \n", m).expect("fmt");
        }

        Ok(())
    }
}

fn get_num(v: &Vec<&str>, index: usize) -> usize {
    return v
        .get(index)
        .expect("Index must exist")
        .parse()
        .expect("Number can be parsed");
}

impl Input {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }

    fn seeds_to_ranges(&self) -> Vec<(usize, usize)> {
        let mut i = self.seeds.iter();

        let mut ranges: Vec<(usize, usize)> = Vec::new();

        loop {
            let s1 = match i.next() {
                Some(s) => s,
                None => break,
            };

            let s2 = match i.next() {
                Some(s) => s,
                None => break,
            };

            ranges.push((*s1, *s2));
        }

        ranges
    }

    fn parse(str: String) -> Input {
        let mut inp = Input::new();
        let mut in_map: bool = false;

        let mut cur_maps: Map = Map::new(String::from(""));

        for line in str.split("\n") {
            if line.trim().is_empty() {
                if in_map {
                    cur_maps.maps.sort();
                    inp.maps.push(cur_maps);
                    cur_maps = Map::empty();
                }
                in_map = false;
                continue;
            }

            if line.contains("seeds:") {
                let seeds: Vec<usize> = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                inp.seeds = seeds;
            }

            if line.contains(" map:") {
                cur_maps = Map::new(line.replace(" map:", "").to_string());
                in_map = true;
                continue;
            }

            if in_map {
                let s = line.split_whitespace().collect::<Vec<_>>();
                cur_maps.maps.push(MapDescription::new(
                    get_num(&s, 0),
                    get_num(&s, 1),
                    get_num(&s, 2),
                ))
            }
        }

        if in_map {
            cur_maps.maps.sort();
            inp.maps.push(cur_maps);
        }

        inp
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::Input;

    #[test]
    fn p1_single_seed() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        println!("{}", inp);

        let mut d: usize = *inp.seeds.get(0).unwrap();
        for map in &inp.maps {
            print!("= Source ({}) {}", &map.name, d);
            d = map.get_dest(d);
            print!(" maps to dest {}\n", d);
        }

        // for seed in inp.seeds {
        //     let mut d = seed;
        //     for map in &inp.maps {
        //         print!("= Source {}", d);
        //         d = map.get_dest(d);
        //         print!(" maps to dest {}\n", d);
        //     }
        // }
    }

    #[test]
    fn p1_all_seeds() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        println!("{}", inp);

        let mut lowest = usize::MAX;

        for seed in inp.seeds {
            let mut d: usize = seed;
            println!("\nSEED:{}\n", d);
            for map in &inp.maps {
                print!("= Source ({}) {}", &map.name, d);
                d = map.get_dest(d);
                print!(" maps to dest {}\n", d);
            }
            lowest = lowest.min(d);
        }

        println!("Lowest location: {}", lowest);
    }

    #[test]
    fn p2_single_range() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        println!("{}", inp);

        // let mut lowest = usize::MAX;

        let ranges = inp.seeds_to_ranges();

        println!("Ranges: {:?}", ranges);

        for i in 14..15 {
            let (mut r, mut rsize) = (i, 1);

            let mut r_ranges = Vec::new();

            println!("========First range: ({},{})", r, rsize);

            // r_ranges.push((**r, **rsize));

            for map in &inp.maps {
                println!("Look into map: {}", map);
                let rs = map.get_des_by_range(r, rsize);

                let expected = map.get_dest(r);

                println!("Expected: {}", expected);
                println!("Found matching ranges: {:?}", rs);

                let m = rs.iter().find(|first_range| first_range.0 == expected);

                if m.is_none() {
                    let diff = rs.iter().map(|r| (r.0, expected)).collect::<Vec<_>>();
                    println!("DIFF = {:?}, map={}", diff, map.name);
                }

                assert!(m.is_some());

                if let Some(mr) = m {
                    r = mr.0;
                    rsize = mr.1;
                }

                r_ranges.push(rs);
                // break;
            }
        }
    }

    #[test]
    fn p2_full_test_one_range() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        // println!("{}", inp);

        // let mut lowest = usize::MAX;

        // let mut ranges: Vec<(usize, usize)> = inp.seeds_to_ranges();
        let mut ranges: Vec<(usize, usize)> = vec![(55, 13)];
        for map in &inp.maps {
            println!("Look into map: {}", map);

            let mut map_ranges: Vec<(usize, usize)> = Vec::new();

            for ran in ranges {
                let mut rs = map.get_des_by_range(ran.0, ran.1);
                map_ranges.append(&mut rs);
            }

            ranges = map_ranges;
            println!("New ranges: {:?}", ranges);
        }

        let mut m = usize::MAX;

        for r in &ranges {
            m = m.min(r.0);
        }

        println!("=Ranges: {:?}", ranges);
        println!("=Min: {}", m);
    }

    #[test]
    fn p2_full_test_single_ranges() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        println!("{}", inp);

        // let mut lowest = usize::MAX;

        // let mut ranges: Vec<(usize, usize)> = inp.seeds_to_ranges();
        let mut ranges: Vec<(usize, usize)> = vec![(55, 1), (79, 1), (14, 1), (13, 1)];
        for map in &inp.maps {
            println!("Look into map: {}", map);

            let mut map_ranges: Vec<(usize, usize)> = Vec::new();

            for ran in ranges {
                let mut rs = map.get_des_by_range(ran.0, ran.1);
                map_ranges.append(&mut rs);
            }

            ranges = map_ranges;
        }

        let mut m = usize::MAX;

        for r in &ranges {
            m = m.min(r.0);
        }

        println!("=Ranges: {:?}", ranges);
        println!("=Min: {}", m);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d5.txt").expect("");
        let inp = Input::parse(contents);

        println!("{}", inp);

        // let mut lowest = usize::MAX;

        let mut ranges: Vec<(usize, usize)> = inp.seeds_to_ranges();

        println!("Inital ranges: {:?}", ranges);

        for map in &inp.maps {
            println!("\nLook into map: {}", map);

            let mut map_ranges: Vec<(usize, usize)> = Vec::new();

            for ran in ranges {
                let mut rs = map.get_des_by_range(ran.0, ran.1);
                map_ranges.append(&mut rs);
            }

            ranges = map_ranges;

            println!("New ranges: {:?}", ranges);
        }

        let mut m = usize::MAX;

        for r in &ranges {
            m = m.min(r.0);
        }

        println!("=Ranges: {:?}", ranges);
        println!("=Min: {}", m);
    }
    #[test]
    fn extra_improvement() {
        let start = 10;
        let count = 10;

        let range: std::ops::Range<i32> = start..count + start;

        assert!(range.contains(&10));
        assert!(range.contains(&15));
        assert!(!range.contains(&20));
    }
}
