// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day2");

#[cfg(not(feature = "day2"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(2, 1)
}

#[cfg(not(feature = "day2"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(2, 2)
}

// v START SOLUTION v

#[cfg(feature = "day2")]
pub fn part1(source: String) -> i32 {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;
    source
    .split("\r\n")
    .for_each(
        |line| {
            let tokens: Vec<_> = line.splitn(2, " ").collect();
            let amount: i32 = tokens[1].parse().unwrap();
            match tokens[0] {
                "up" => depth -= amount,
                "down" => depth += amount,
                "forward" => horiz += amount,
                _ => unreachable!()
            }
        }
    );
    depth * horiz
}

#[cfg(feature = "day2")]
pub fn part2(source: String) -> i32 {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;
    let mut aim: i32 = 0;
    source
    .split("\r\n")
    .for_each(
        |line| {
            let tokens: Vec<_> = line.splitn(2, " ").collect();
            let amount: i32 = tokens[1].parse().unwrap();
            match tokens[0] {
                "up" => aim -= amount,
                "down" => aim += amount,
                "forward" => {
                    horiz += amount;
                    depth += amount * aim;
                }
                _ => unreachable!()
            }
        }
    );
    depth * horiz
}
