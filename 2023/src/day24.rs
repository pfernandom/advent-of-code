#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix6, RowVector6, Vector3, Vector6};
    use std::{fs, str::FromStr};

    #[derive(Debug)]
    struct Coord {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Debug)]
    struct Position<T: Copy + Clone + PartialEq> {
        x: T,
        y: T,
        z: T,
        vx: T,
        vy: T,
        vz: T,
    }

    impl<
            T: Copy
                + Clone
                + core::fmt::Debug
                + PartialEq
                + FromStr
                + std::ops::Neg<Output = T>
                + std::ops::Sub<Output = T>
                + std::ops::Mul<Output = T>
                + std::ops::Div<Output = T>
                + std::ops::Add<Output = T>,
        > Position<T>
    {
        fn parse(str: &str) -> Self
        where
            <T as FromStr>::Err: core::fmt::Debug,
        {
            let (left, right) = str.split_once(" @ ").unwrap();

            let left = left
                .trim()
                .split(",")
                .map(|e| e.trim())
                .map(|e| T::from_str(e).unwrap())
                .collect::<Vec<_>>();

            let right = right
                .trim()
                .split(",")
                .map(|e| e.trim())
                .map(|e| T::from_str(e).unwrap())
                .collect::<Vec<_>>();

            Position {
                x: left[0],
                y: left[1],
                z: left[2],
                vx: right[0],
                vy: right[1],
                vz: right[2],
            }
        }

        fn clone_after_time(&self, time: T) -> Position<T> {
            Self {
                x: self.x + time * self.vx,
                y: self.y + time * self.vy,
                z: self.z + time * self.vz,
                vx: self.vx,
                vy: self.vy,
                vz: self.vz,
            }
        }

        fn to_line(&self) -> (T, T) {
            let m = self.vy / self.vx;
            let b = -m * self.x + self.y;
            (m, b)
        }

        fn get_time_for_other_x(&self, other_x: T) -> T {
            (other_x - self.x) / self.vx
        }

        fn intersection(&self, other: &Self) -> Coord
        where
            f64: From<T>,
        {
            // ax+by+cz = 0
            let (m1, lb1) = self.to_line();
            let (m2, lb2) = other.to_line();

            let a1: f64 = m1.into();
            let a2: f64 = m2.into();
            let b1 = -1.0;
            let b2 = -1.0;
            let c1: f64 = lb1.into();
            let c2: f64 = lb2.into();

            let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
            let y = (a2 * c1 - a1 * c2) / (a1 * b2 - a2 * b1);

            Coord { x, y, z: 0.0 }
        }
    }

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d24.txt").expect("");

        let range = 7.0..27.0 + 0.001;
        // let range = 200000000000000.0..400000000000000.0 + 0.0000000001;

        let data = contents
            .lines()
            .map(|e| Position::<f64>::parse(e))
            .collect::<Vec<_>>();

        let mut collisions = 0;
        for (i1, p1) in data.iter().enumerate() {
            for i2 in i1..data.len() {
                if i1 != i2 {
                    let p2 = data.get(i2).unwrap();
                    let inter = p1.intersection(p2);
                    println!("{:?}", inter);
                    if range.contains(&inter.x)
                        && range.contains(&inter.y)
                        && p1.get_time_for_other_x(inter.x) > 0.0
                        && p2.get_time_for_other_x(inter.x) > 0.0
                    {
                        collisions += 1;
                    }
                }
            }
        }

        println!("collisions: {}", collisions);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d24.txt").expect("");

        // // let range = 7.0..27.0 + 0.001;
        // let range = 200000000000000.0..400000000000000.0 + 0.0000000001;

        let data = contents
            .lines()
            .map(|e| Position::<i64>::parse(e))
            .collect::<Vec<_>>();

        let mut ma = Vec::new();
        let mut mb = Vec::new();

        for i in 0..2 {
            let j = i + 1;

            let hi = data.get(i).unwrap();
            let hj = data.get(j).unwrap();

            let a = Vector3::new(hi.x, hi.y, hi.z).cross(&Vector3::new(hi.vx, hi.vy, hi.vz));
            let b = Vector3::new(hj.x, hj.y, hj.z).cross(&Vector3::new(hj.vx, hj.vy, hj.vz));

            mb.append(
                &mut a
                    .iter()
                    .zip(b.iter())
                    .map(|(ia, ib)| (ia - ib) as f64)
                    .rev()
                    .collect::<Vec<_>>(),
            );

            let x = hi.vy - hj.vy;
            let y = -(hi.vx - hj.vx);
            let z = 0;
            let vx = -(hi.y - hj.y);
            let vy = hi.x - hj.x;
            let vz = 0;
            ma.push(RowVector6::from_vec(
                [x, y, z, vx, vy, vz].iter().map(|e| *e as f64).collect(),
            ));

            let x = -(hi.vz - hj.vz);
            let y = 0;
            let z = hi.vx - hj.vx;
            let vx = hi.z - hj.z;
            let vy = 0;
            let vz = -(hi.x - hj.x);

            ma.push(RowVector6::from_vec(
                [x, y, z, vx, vy, vz].iter().map(|e| *e as f64).collect(),
            ));

            let x = 0;
            let y = hi.vz - hj.vz;
            let z = -(hi.vy - hj.vy);
            let vx = 0;
            let vy = -(hi.z - hj.z);
            let vz = hi.y - hj.y;

            ma.push(RowVector6::from_vec(
                [x, y, z, vx, vy, vz].iter().map(|e| *e as f64).collect(),
            ));
        }

        let ma = Matrix6::from_rows(&ma);
        let mb = Vector6::from_vec(mb);

        let decomp = ma.lu();
        let x = decomp.solve(&mb).expect("Linear resolution failed.");

        let res = x[0] + x[1] + x[2];
        let res = res.round();
        println!("{}", res);
    }

    #[test]
    fn test_speed() {
        let p = Position::parse("20, 19, 15 @ 1, -5, -3");

        println!("{:?}", p);

        println!("{:?}", p.clone_after_time(1.0));
    }

    #[test]
    fn test_line() {
        let p = Position::parse("6, 4, 15 @ 1, -5, -3");

        println!("{:?}", p);

        println!("{:?}", p.clone_after_time(1.0));

        p.to_line();
    }
}
