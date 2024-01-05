#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use pest::{iterators::Pair, Parser};
    use pest_derive::Parser;

    use std::ops::Range;
    use std::{collections::HashMap, fs};

    // #[grammar = "ident.pest"]
    #[derive(Parser)]
    #[grammar = "rules.pest"]
    struct IdentParser;

    pub trait FromNode {
        fn parse(rule: &Pair<'_, Rule>) -> Option<Self>
        where
            Self: Sized;
    }

    type PartName = String;
    // type CondType = String;
    type WorkFlowRes = String;

    #[derive(Debug)]
    struct WorkFlow {
        name: String,
        steps: Vec<CondType>,
        default: PartName,
    }

    #[derive(Debug)]
    enum CondType {
        LT {
            value: i32,
            part_name: PartName,
            res_wf: String,
        },
        GT {
            value: i32,
            part_name: PartName,
            res_wf: String,
        },
    }

    impl CondType {
        fn new(cond_type: &str, part_name: &str, value: &str, wf_name: &str) -> Self {
            let value = i32::from_str_radix(value, 10).unwrap();

            match cond_type {
                "<" => CondType::LT {
                    value,
                    part_name: part_name.to_string(),
                    res_wf: wf_name.to_string(),
                },
                ">" => CondType::GT {
                    value,
                    part_name: part_name.to_string(),
                    res_wf: wf_name.to_string(),
                },
                _ => unreachable!(),
            }
        }

        fn negate(&self) -> CondType {
            match self {
                CondType::LT {
                    value,
                    part_name,
                    res_wf,
                } => CondType::GT {
                    value: *value - 1,
                    part_name: part_name.clone(),
                    res_wf: res_wf.clone(),
                },
                CondType::GT {
                    value,
                    part_name,
                    res_wf,
                } => CondType::LT {
                    value: *value + 1,
                    part_name: part_name.clone(),
                    res_wf: res_wf.clone(),
                },
            }
        }

        fn wf_name(&self) -> PartName {
            match self {
                CondType::LT { part_name, .. } => part_name.clone(),
                CondType::GT { part_name, .. } => part_name.clone(),
            }
        }

        fn get_resulting_wf(&self) -> String {
            match self {
                CondType::LT { res_wf, .. } => res_wf.clone(),
                CondType::GT { res_wf, .. } => res_wf.clone(),
            }
        }

        fn apply(&self, other: &HashMap<String, i32>) -> Option<PartName> {
            other
                .get(&self.wf_name())
                .map(|part_value| match self {
                    CondType::LT { value, res_wf, .. } => {
                        if *part_value < *value {
                            Some(res_wf.clone())
                        } else {
                            None
                        }
                    }
                    CondType::GT { value, res_wf, .. } => {
                        if part_value > value {
                            Some(res_wf.clone())
                        } else {
                            None
                        }
                    }
                })
                .flatten()
        }
    }

    impl WorkFlow {
        fn empty() -> Self {
            Self {
                name: String::new(),
                steps: Vec::new(),
                default: String::new(),
            }
        }
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                steps: Vec::new(),
                default: String::new(),
            }
        }

        fn apply(&self, other: &HashMap<String, i32>) -> Option<PartName> {
            for step in &self.steps {
                let applied = step.apply(other);
                if applied.is_some() {
                    return applied;
                }
            }

            // println!("- Default: {}", self.default);
            Some(self.default.clone())
        }
    }

    fn parse_input() -> (HashMap<String, WorkFlow>, Vec<HashMap<String, i32>>) {
        let contents = fs::read_to_string("./problems/d19.txt").expect("");

        let pairs =
            IdentParser::parse(Rule::input, &contents.as_str()).unwrap_or_else(|e| panic!("{}", e));

        let mut workflows: HashMap<String, WorkFlow> = HashMap::new();
        let mut examples = Vec::new();
        for pair in pairs {
            for inner in pair.into_inner() {
                // println!("{}", inner.as_str());
                match inner.as_rule() {
                    Rule::rule_block => {
                        // println!("Rule block: {:?}", inner.as_str());
                        let mut workflow = WorkFlow::empty();
                        for inner in inner.into_inner() {
                            match inner.as_rule() {
                                Rule::work_flow_name => {
                                    workflow.name = inner.as_str().to_string();
                                }
                                Rule::cond => {
                                    let mut parth_name = "";
                                    let mut cond_type = "";
                                    let mut value = "";
                                    let mut wf_name = "";

                                    // println!("Condition: {:?}", inner.as_str());
                                    for inner in inner.into_inner() {
                                        match inner.as_rule() {
                                            Rule::part_name => {
                                                // println!("part_name: {:?}", inner.as_str());
                                                parth_name = inner.as_str();
                                            }
                                            Rule::cond_type => {
                                                // println!("cond_type: {:?}", inner.as_str());
                                                cond_type = inner.as_str();
                                            }
                                            Rule::num_literal => {
                                                value = inner.as_str();
                                            }
                                            Rule::work_flow_name => {
                                                // println!("work_flow_name: {:?}", inner.as_str());
                                                wf_name = inner.as_str();
                                            }
                                            _ => {}
                                        }
                                    }

                                    workflow
                                        .steps
                                        .push(CondType::new(cond_type, parth_name, value, wf_name));
                                }
                                Rule::default_label => {
                                    workflow.default = inner.as_str().to_string()
                                }
                                _ => {}
                            }
                        }

                        workflows.insert(workflow.name.clone(), workflow);
                    }
                    Rule::parts => {
                        let mut parts = HashMap::new();
                        for inner in inner.into_inner() {
                            match inner.as_rule() {
                                Rule::part => {
                                    let mut part_name = "";
                                    let mut value = "";
                                    for inner in inner.into_inner() {
                                        match inner.as_rule() {
                                            Rule::part_name => part_name = inner.as_str(),
                                            Rule::num_literal => value = inner.as_str(),
                                            _ => {}
                                        }
                                    }
                                    parts.insert(
                                        part_name.to_string(),
                                        i32::from_str_radix(value, 10).unwrap(),
                                    );
                                }
                                _ => {}
                            }
                        }
                        examples.push(parts);
                    }
                    _ => {}
                }
            }
        }
        (workflows, examples)
    }

    #[test]
    fn p1() {
        let (workflows, examples) = parse_input();

        // println!("{:?}", workflows);
        // println!("{:?}", examples);

        let mut res = 0;

        for example in examples {
            // println!(": {:?}", example);
            let mut nf = String::from("in");
            while let Some(w) = workflows.get(&nf) {
                let next_flow = w.apply(&example).unwrap();
                // println!("next_flow={}", next_flow);
                nf = next_flow;
            }
            println!("====={}", nf);

            if nf == "A" {
                println!("Accepted: {:?}", example);
                res += example.values().fold(0, |acc, el| acc + el)
            }
        }

        println!("Result: {}", res);
    }

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
    fn count_combinations(n: u64, r: u64) -> u64 {
        (n - r + 1..=n).product::<u64>() / factorial(r)
    }

    #[test]
    fn test_comb() {
        //         = a
        // [ARange(2005..4000)], count:1995
        // = x
        // [ARange(1415..4000)], count:2585
        // = m
        // [ARange(1..2091)], count:2090
        // = s
        // [ARange(1350..4000)], count:2650
        // let num_per_rating = 4000;
        // let num_ratings = 4;

        println!("{}", count_combinations(1995 + 2585 + 2090 + 2650, 4))
    }

    fn traverse(
        name: String,
        workflows: &HashMap<String, WorkFlow>,
        level: usize,
        range_map: &mut RangeMap,
    ) -> i64 {
        if name.as_str() == "A" || name.as_str() == "R" {
            // println!("{}{}", sp, name);
            if name.as_str() == "A" {
                return range_map.count();
            } else {
                return 0;
            }
        }
        let w = workflows.get(name.as_str()).unwrap();

        let mut s = 0;

        let mut range_map = range_map.clone();

        for cond_type in &w.steps {
            let (mut true_map, false_map) = range_map.split_on(cond_type).unwrap();

            s += traverse(
                cond_type.get_resulting_wf().clone(),
                workflows,
                level + 1,
                &mut true_map,
            );

            range_map = false_map;
        }

        s + traverse(w.default.clone(), workflows, level + 1, &mut range_map)
    }

    #[derive(Debug, Clone)]
    struct ARange(Range<i32>);

    impl ARange {
        // []

        fn try_merge(&self, other: &ARange) -> Option<ARange> {
            if self.0.contains(&other.0.start)
                || self.0.contains(&other.0.end)
                || other.0.contains(&self.0.start)
                || other.0.contains(&self.0.end)
            {
                let r = ARange(self.0.start.min(other.0.start)..self.0.end.max(other.0.end));
                Some(r)
            } else {
                None
            }
        }

        fn split_on(&self, cond: &CondType) -> (ARange, ARange) {
            let start = self.0.start;
            let end = self.0.end;
            match cond {
                CondType::LT { value, .. } => (ARange(start..*value), ARange(*value..end)), // vec![(ARange(start..*value), true), (ARange(*value..end), false)],
                CondType::GT { value, .. } => (ARange(*value + 1..end), ARange(start..*value + 1)), // vec![(ARange(start..*value), false), (ARange(*value..end), true)],
            }
        }
    }

    #[derive(Debug, Clone)]
    struct RangeMap {
        m: HashMap<String, Vec<(ARange, bool)>>,
    }

    impl RangeMap {
        fn new(entries: Vec<(&str, Vec<(ARange, bool)>)>) -> Self {
            Self {
                m: entries
                    .iter()
                    .map(|(key, val)| (key.to_string(), val.clone()))
                    .collect(),
            }
        }

        fn split_on(&mut self, cond: &CondType) -> Option<(RangeMap, RangeMap)> {
            let cond_name = cond.wf_name();
            match self.m.get(&cond_name) {
                Some(r) => {
                    let (true_ranges, false_ranges): (Vec<(ARange, bool)>, Vec<(ARange, bool)>) = r
                        .iter()
                        .map(|r| r.0.split_on(&cond))
                        .map(|t| ((t.0, true), (t.1, false)))
                        .unzip();
                    let mut r1 = self.clone();
                    let mut r2 = self.clone();

                    r1.m.insert(cond_name.clone(), true_ranges);
                    r2.m.insert(cond_name, false_ranges);
                    Some((r1, r2))
                }
                None => None,
            }
        }

        fn count(&self) -> i64 {
            self.m
                .values()
                .flat_map(|v| v)
                .map(|t| t.0.clone())
                .map(|r| r.0.len() as i64)
                .fold(1, |acc, r| acc * r)
        }
    }

    #[test]
    fn p2() {
        let (workflows, _) = parse_input();

        let org_val = (ARange(1..4000 + 1), true);

        let mut r_map = RangeMap::new(vec![
            ("x", vec![org_val.clone()]),
            ("m", vec![org_val.clone()]),
            ("a", vec![org_val.clone()]),
            ("s", vec![org_val.clone()]),
        ]);
        let res = traverse("in".to_string(), &workflows, 0, &mut r_map);

        println!("{}", (4000 as i64).pow(4));
        println!("{:?}", res);
    }

    #[test]
    fn ranges() {
        let org_val = (ARange(1..4000 + 1), true);

        let mut r_map = RangeMap::new(vec![
            ("x", vec![org_val.clone()]),
            ("m", vec![org_val.clone()]),
            ("a", vec![org_val.clone()]),
            ("s", vec![org_val.clone()]),
        ]);

        let (mut true_map, false_map) = r_map
            .split_on(&CondType::GT {
                value: 3,
                part_name: "x".to_string(),
                res_wf: "y".to_string(),
            })
            .unwrap();

        let (true_map, false_map2) = true_map
            .split_on(&CondType::LT {
                value: 32,
                part_name: "a".to_string(),
                res_wf: "y".to_string(),
            })
            .unwrap();
        let total_count = true_map.count() + false_map.count() + false_map2.count();

        println!("{}", total_count);
        println!("{}", (4000 as i64).pow(4));
    }
}
