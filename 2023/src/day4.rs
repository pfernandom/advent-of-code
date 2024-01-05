#[cfg(test)]
mod tests {

    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d4.txt").expect("");
        let tickets = contents
            .split("\n")
            .map(|l| {
                println!("{}", l);
                l.trim()
                    .split_once(':')
                    .expect("Delimiter :")
                    .1
                    .split_once("|")
                    .unwrap()
            })
            .map(|(l1, l2)| {
                (
                    l1.trim()
                        .split(" ")
                        .filter(|l| !l.trim().is_empty())
                        .collect::<Vec<_>>(),
                    l2.trim()
                        .split(" ")
                        .filter(|l| !l.trim().is_empty())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        println!("{:?}", tickets);

        let base: i32 = 2;
        let mut sum = 0;

        for (winners, tickets) in tickets {
            let winners_hash = winners.iter().map(|t| *t).collect::<HashSet<_>>();

            let tickets_hash = tickets.iter().map(|t| *t).collect::<HashSet<_>>();

            let winner_tickets = winners_hash.intersection(&tickets_hash).collect::<Vec<_>>();

            let count: u32 = winner_tickets.len().try_into().unwrap();

            if count > 0 {
                let winner_counts = base.pow(count - 1);
                println!("{:?}, {}", winner_tickets, winner_counts);
                sum += winner_counts;
            }
        }

        println!("Res: {}", sum);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d4.txt").expect("");
        let tickets = contents
            .split("\n")
            .map(|l| {
                println!("{}", l);
                l.trim()
                    .split_once(':')
                    .expect("Delimiter :")
                    .1
                    .split_once("|")
                    .unwrap()
            })
            .map(|(l1, l2)| {
                (
                    l1.trim()
                        .split(" ")
                        .filter(|l| !l.trim().is_empty())
                        .collect::<Vec<_>>(),
                    l2.trim()
                        .split(" ")
                        .filter(|l| !l.trim().is_empty())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let mut copies = HashMap::new();

        for (i, (winners, tickets)) in tickets.iter().enumerate() {
            let winners_hash = winners.iter().map(|t| *t).collect::<HashSet<_>>();

            let tickets_hash = tickets.iter().map(|t| *t).collect::<HashSet<_>>();

            let winner_tickets = winners_hash.intersection(&tickets_hash).collect::<Vec<_>>();

            copies.insert(i, *copies.get(&i).unwrap_or(&0) + 1);

            let copies_len = *copies.get(&i).unwrap_or(&1);

            let count = winner_tickets.len();

            if count > 0 {
                for c in i + 1..i + 1 + count {
                    copies.insert(c, copies.get(&c).unwrap_or(&0) + copies_len);
                }
            }
        }

        let r: usize = copies
            .values()
            .map(|n| n.clone())
            .reduce(|a, b| a + b)
            .unwrap();
        println!("Res: {}", r);
    }
}
