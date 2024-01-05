#[cfg(test)]
mod tests {

    use crate::read_file;

    fn p1(str: String) -> String {
        let mut first: char = '_';
        let mut second: char = '_';
        for (i, c) in str.chars().filter(|c| c.is_numeric()).enumerate() {
            if i == 0 {
                first = c;
            }
            second = c;
        }

        return format!("{}{}", first, second);
    }

    #[test]
    fn day1p1() {
        let mut result: u32 = 0;
        for line in read_file::read_file("./problems/p1.txt".to_string()) {
            let locations = p1(line);
            let sl = locations.as_str();
            println!("{}", locations);
            result += u32::from_str_radix(sl, 10).unwrap();
        }
        println!("P1: {}", result);
    }

    #[derive(PartialEq)]
    enum State {
        Init,
        One,
        Two,
        Three,
        TwoOrThree,
        Four,
        Five,
        FourOrFive,
        Six,
        Seven,
        SixOrSeven,
        Eight,
        Nine,
    }

    impl State {
        pub fn is_complete(&self, index: usize, c: &char) -> bool {
            return match self {
                State::Init => false,
                State::One => index == 2 && *c == 'e',
                State::Two => index == 2 && *c == 'o',
                State::Three => index == 4 && *c == 'e',
                State::TwoOrThree => false,
                State::Four => index == 3 && *c == 'r',
                State::Five => index == 3 && *c == 'e',
                State::FourOrFive => false,
                State::Six => index == 2 && *c == 'x',
                State::Seven => index == 4 && *c == 'n',
                State::SixOrSeven => false,
                State::Eight => index == 4 && *c == 't',
                State::Nine => index == 3 && *c == 'e',
            };
        }

        pub fn to_digit(&self) -> u32 {
            return match self {
                State::One => 1,
                State::Two => 2,
                State::Three => 3,
                State::TwoOrThree => panic!("TwoOrThree"),
                State::Four => 4,
                State::Five => 5,
                State::FourOrFive => panic!("FourOrFive"),
                State::Six => 6,
                State::Seven => 7,
                State::SixOrSeven => panic!("SixOrSeven"),
                State::Eight => 8,
                State::Nine => 9,
                _ => panic!("Other"),
            };
        }

        pub fn next_state(&self, index: usize, c: &char) -> Option<State> {
            let m = |i: &&char| {
                // println!("{} index={}, i={} c={}", self.to_digit(), index, i, c);
                let res = *i == c;
                return res;
            };

            return match self {
                State::Init => match c {
                    'o' => Some(State::One),
                    't' => Some(State::TwoOrThree),
                    'f' => Some(State::FourOrFive),
                    's' => Some(State::SixOrSeven),
                    'e' => Some(State::Eight),
                    'n' => Some(State::Nine),
                    _ => None,
                },
                State::One => ['o', 'n', 'e'].get(index).filter(m).map(|_| State::One),
                State::Two => ['t', 'w', 'o'].get(index).filter(m).map(|_| State::Two),
                State::Three => ['t', 'h', 'r', 'e', 'e']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Three),
                State::TwoOrThree => {
                    if index != 1 {
                        None
                    } else if *c == 'w' {
                        Some(State::Two)
                    } else if *c == 'h' {
                        Some(State::Three)
                    } else {
                        None
                    }
                }
                State::Four => ['f', 'o', 'u', 'r']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Four),
                State::Five => ['f', 'i', 'v', 'e']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Five),
                State::FourOrFive => {
                    if index != 1 {
                        None
                    } else if *c == 'o' {
                        Some(State::Four)
                    } else if *c == 'i' {
                        Some(State::Five)
                    } else {
                        None
                    }
                }
                State::Six => ['s', 'i', 'x'].get(index).filter(m).map(|_| State::Six),
                State::Seven => ['s', 'e', 'v', 'e', 'n']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Seven),
                State::SixOrSeven => {
                    if index != 1 {
                        None
                    } else if *c == 'i' {
                        Some(State::Six)
                    } else if *c == 'e' {
                        Some(State::Seven)
                    } else {
                        None
                    }
                }
                State::Eight => ['e', 'i', 'g', 'h', 't']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Eight),
                State::Nine => ['n', 'i', 'n', 'e']
                    .get(index)
                    .filter(m)
                    .map(|_| State::Nine),
            };
        }
    }

    fn p2(str: String) -> String {
        let map = str.chars().collect::<Vec<_>>();

        let mut states: Vec<(State, usize, usize)> = vec![(State::Init, 0, 0)];
        // let mut c_index: usize = 0;

        let mut digits: Vec<u32> = Vec::new();

        // let mut i = 0;

        while !states.is_empty() {
            let (state, c_index, i) = states.pop().unwrap();
            let c = match map.get(i) {
                Some(s) => s,
                None => break,
            };
            if c.is_digit(10) {
                let d = c.to_digit(10).unwrap().into();
                digits.push(d);
                // println!("Found {}", d);
                states.insert(0, (State::Init, 0, i + 1));
                continue;
            }
            if state.is_complete(c_index, c) {
                // println!("Found {}", state.to_digit());
                digits.push(state.to_digit());
                states.insert(0, (State::Init, 0, i));
            } else {
                let new_state = match state.next_state(c_index, c) {
                    None => State::Init,
                    Some(s) => s,
                };
                if state != State::Init {
                    states.insert(0, (State::Init, 0, i));
                }
                if new_state != State::Init {
                    states.insert(0, (new_state, c_index + 1, i + 1));
                }
            }

            if states.is_empty() && i < map.len() {
                states.push((State::Init, 0, i + 1));
            }
        }

        return digits
            .get(0)
            .map(|first| {
                digits.last().map(|last| {
                    let r = format!("{}{}", first, last);
                    r
                })
            })
            .flatten()
            .expect("No digits found");
    }

    #[test]
    fn day1p2() {
        let mut result: u32 = 0;
        for line in read_file::read_file("./problems/p2.txt".to_string()) {
            let locations = p2(line.clone());
            let sl = locations.as_str();
            let digs = u32::from_str_radix(sl, 10).unwrap();
            println!("// {} \t {}", digs, line);
            result += digs;
        }
        println!("P2: {}", result);
    }

    #[test]
    fn day1p2alt() {
        let mut result: u64 = 0;
        for line in read_file::read_file("./problems/p2.txt".to_string()) {
            let sl = line.as_str();
            let sl1 = sl
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");

            let sl = p1(sl1.clone());
            let digs = u64::from_str_radix(sl.as_str(), 10).unwrap();
            println!("// {} \t {}", digs, line);
            result += digs;
        }
        println!("P2: {}", result);
    }
}
