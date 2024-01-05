#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use num_integer::lcm;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        fmt::Display,
        fs,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ModuleType {
        FlipFlop(bool),
        Conjunction,
        Broadcaster,
        Other,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Module {
        name: String,
        module_type: ModuleType,
        targets: Vec<String>,
        initial_state: bool,
        last_pulse_received: HashMap<String, Pulse>,
    }

    impl Display for Module {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let last_received = self
                .last_pulse_received
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(",");
            let md = match self.module_type {
                ModuleType::FlipFlop(st) => {
                    format!("%({})", if st == false { "off" } else { "on" })
                }
                ModuleType::Conjunction => format!("&({})", last_received),
                ModuleType::Broadcaster => format!(""),
                ModuleType::Other => format!(""),
            };
            write!(f, "{} {} {{ {} }}", md, self.name, self.initial_state)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Pulse {
        High,
        Low,
    }

    impl Display for Pulse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Pulse::High => write!(f, "High"),
                Pulse::Low => write!(f, "Low"),
            }
        }
    }

    impl Module {
        fn from_str(s: &str, targets: Vec<&str>) -> Self {
            let targets = targets
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let module_type = if s.starts_with("%") {
                ModuleType::FlipFlop(false)
            } else if s.starts_with("&") {
                ModuleType::Conjunction
            } else if s.starts_with("broadcaster") {
                ModuleType::Broadcaster
            } else {
                ModuleType::Other
            };

            Self {
                name: s.replace("%", "").replace("&", "").to_string(),
                module_type,
                targets,
                initial_state: true,
                last_pulse_received: HashMap::new(),
            }
        }
    }

    impl Module {
        fn get_name(&self) -> &String {
            &self.name
        }

        fn fire(&self, pulse: Pulse) -> VecDeque<(String, Pulse)> {
            self.targets
                .iter()
                .map(|n| (n.clone(), pulse.clone()))
                .collect()
        }

        fn conj_pulse_to_fire(&self) -> Pulse {
            if self.last_pulse_received.values().all(|v| *v == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            }
        }

        fn process(&mut self, from: &String, pulse: Pulse) -> VecDeque<(String, Pulse)> {
            // println!("Process: {:?}, pulse:{:?}", self, pulse);

            match self.module_type {
                ModuleType::FlipFlop(curr_state) => match pulse {
                    Pulse::High => VecDeque::new(),
                    Pulse::Low => {
                        self.module_type = ModuleType::FlipFlop(!curr_state);
                        self.initial_state = !curr_state == false;
                        self.fire(if curr_state == false {
                            Pulse::High
                        } else {
                            Pulse::Low
                        })
                    }
                },
                ModuleType::Conjunction => {
                    self.last_pulse_received.insert(from.clone(), pulse.clone());
                    self.initial_state =
                        self.last_pulse_received.values().all(|v| *v == Pulse::Low);
                    self.fire(self.conj_pulse_to_fire())
                }
                ModuleType::Broadcaster => self.fire(pulse),
                ModuleType::Other => VecDeque::new(),
            }
        }
    }

    fn process(
        module: &Module,
        from: &String,
        pulse: Pulse,
    ) -> (Module, VecDeque<(String, Pulse)>) {
        let mut m = module.clone();
        let pulses = m.process(from, pulse);

        (m, pulses)
    }

    fn press_button(data: &mut HashMap<String, Module>) -> (i64, i64) {
        let mut high_pulses = 0;
        let mut low_pulses = 1;
        let mut next_pulses = VecDeque::new();

        next_pulses.push_front((
            String::from("button"),
            String::from("broadcaster"),
            Pulse::Low,
        ));

        while let Some((from, next_name, pulse)) = next_pulses.pop_front() {
            match data.get_mut(&next_name) {
                Some(m) => {
                    let more_pulses = m.process(&from, pulse);
                    // data.insert(next_name.clone(), m.clone());
                    // for p in &more_pulses {
                    //     println!("{} -{:?}-> {}", m.get_name(), p.1, p.0);
                    // }

                    for p in more_pulses {
                        if p.1 == Pulse::Low {
                            low_pulses += 1;
                        } else {
                            high_pulses += 1;
                        }
                        next_pulses.push_back((next_name.clone(), p.0, p.1));
                    }
                }
                None => {
                    // println!("{}", next_name);
                }
            }
        }

        (high_pulses, low_pulses)
    }

    fn press_button_p2(
        data: &mut HashMap<String, Module>,
        pres: &HashSet<&String>,
    ) -> Option<String> {
        let mut next_pulses = VecDeque::new();

        next_pulses.push_front((
            String::from("button"),
            String::from("broadcaster"),
            Pulse::Low,
        ));

        while let Some((from, next_name, pulse)) = next_pulses.pop_front() {
            match data.get_mut(&next_name) {
                Some(m) => {
                    if pulse == Pulse::Low && next_name == "rx" {
                        return Some(from);
                    }
                    if pulse == Pulse::High && pres.contains(&from) && next_name.contains("hp") {
                        return Some(from);
                    }

                    let more_pulses = m.process(&from, pulse);

                    for p in more_pulses {
                        next_pulses.push_back((next_name.clone(), p.0, p.1));
                    }
                }
                None => {
                    // println!("{}", next_name);
                }
            }
        }

        None
    }

    fn set_initial_state(data: &mut HashMap<String, Module>) {
        let mut target_to_source: HashMap<String, HashSet<String>> = HashMap::new();
        for (key, val) in data.iter() {
            for t in &val.targets {
                target_to_source
                    .entry(t.clone())
                    .or_insert(HashSet::new())
                    .insert(key.clone());
            }
        }

        for (target, val) in data {
            match target_to_source.get(target) {
                Some(sources) => {
                    for s in sources {
                        val.last_pulse_received.insert(s.to_string(), Pulse::Low);
                    }
                }
                None => {}
            }
        }
    }

    fn get_input() -> HashMap<String, Module> {
        let contents = fs::read_to_string("./problems/d20.txt").expect("");
        contents
            .lines()
            .map(|l| l.split_once(" -> ").unwrap())
            .map(|pair| {
                (
                    pair.0.trim().replace("%", "").replace("&", "").to_string(),
                    Module::from_str(
                        pair.0.trim(),
                        pair.1
                            .trim()
                            .split(",")
                            .map(|e| e.trim())
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    #[test]
    fn p1() {
        let mut data: HashMap<String, Module> = get_input();

        // println!("{:?}", data);

        set_initial_state(&mut data);
        println!("{:?}", data);

        let mut high_pulses = 0;
        let mut low_pulses = 0;

        let total_presses = 1000;
        let mut presses_to_go = 0;
        let mut presses_multiplier = 1;

        for i in 1..total_presses + 1 {
            let (h, l) = press_button(&mut data);
            high_pulses += h;
            low_pulses += l;
            if data.values().all(|v| v.initial_state) {
                if i < total_presses {
                    presses_multiplier = total_presses / i;
                    presses_to_go = total_presses % i;
                }

                println!("Original state after {} button presses", i);
                break;
            }
        }

        high_pulses = high_pulses * presses_multiplier;
        low_pulses = low_pulses * presses_multiplier;

        println!(
            "Emitted Pulses:  high:{}, low:{}, total:{}",
            high_pulses,
            low_pulses,
            high_pulses * low_pulses
        );

        println!("presses_to_go: {}", presses_to_go);
    }

    fn lcm_list(list: Vec<i64>) -> i64 // Restrict T to types that are primitive integers
    {
        list.iter().cloned().fold(1, |acc, x| lcm(acc, x))
    }

    #[test]
    fn p2() {
        let mut data: HashMap<String, Module> = get_input();

        set_initial_state(&mut data);

        let last = data
            .values()
            .find(|m| m.targets.contains(&String::from("rx")))
            .unwrap()
            .clone();

        println!("{}", last);

        let mut pre_last: HashSet<&String> =
            last.last_pulse_received.keys().collect::<HashSet<_>>();
        println!("{:?}", pre_last);

        let mut i = 1;
        let mut founds = Vec::new();
        loop {
            match press_button_p2(&mut data, &pre_last) {
                Some(f) => {
                    println!("Found {} at {}", f, i);
                    pre_last.remove(&f);
                    founds.push(i);
                    // break;
                }
                None => {}
            }
            if pre_last.is_empty() {
                break;
            }
            i += 1;
        }

        println!("{}", lcm_list(founds));
    }

    #[test]
    fn test_flip_flop() {
        let mut rm = Module::from_str("%x", vec!["a1", "a2", "a3"]);
        let pl = String::from("_");
        match &rm.module_type {
            &ModuleType::FlipFlop(state) => {
                assert!(rm.initial_state);
                assert_eq!(state, false)
            }
            _ => unreachable!(),
        }
        let ts = rm.process(&pl, Pulse::High);
        assert!(ts.is_empty());
        match &rm.module_type {
            &ModuleType::FlipFlop(state) => {
                assert!(rm.initial_state);
                assert_eq!(state, false)
            }
            _ => unreachable!(),
        }

        let ts = rm.process(&pl, Pulse::Low);

        match &rm.module_type {
            &ModuleType::FlipFlop(state) => {
                assert_eq!(rm.initial_state, false);
                assert_eq!(state, true)
            }
            _ => unreachable!(),
        }

        assert_eq!(ts.len(), 3);

        let ts = rm.process(&pl, Pulse::Low);
        match &rm.module_type {
            &ModuleType::FlipFlop(state) => {
                assert!(rm.initial_state);
                assert_eq!(state, false)
            }
            _ => unreachable!(),
        }
        assert_eq!(ts.len(), 3);
    }

    #[test]
    fn test_conj() {
        let mut m = Module::from_str("&x", vec!["a1", "a2"]);
        // let pl = String::from("_");
        assert!(m.initial_state);
        assert!(m.last_pulse_received.values().all(|v| *v == Pulse::Low));

        // first, send low pulse
        let ts = m.process(&String::from("a1"), Pulse::Low);
        assert!(m.initial_state);
        assert!(m.last_pulse_received.values().all(|v| *v == Pulse::Low));

        assert!(ts.iter().all(|p| p.1 == Pulse::High));

        // now send high pulse
        let ts = m.process(&String::from("a1"), Pulse::High);
        assert!(!m.initial_state);
        assert!(!m.last_pulse_received.values().all(|v| *v == Pulse::Low));

        assert!(ts.iter().all(|p| p.1 == Pulse::Low))
    }
}
