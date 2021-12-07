// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day7");

#[cfg(not(feature = "day7"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(7, 1)
}

#[cfg(not(feature = "day7"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(7, 2)
}

// v START SOLUTION v

#[cfg(feature = "day7")]
pub fn part1(source: String) -> i64 {
    let mut input: Vec<_> = source.split(",").map(
        |n| n.parse::<i64>().unwrap()
    ).collect();
    let input_slice = input.as_mut_slice();
    input_slice.sort_unstable();

    let accum: Vec<_> = input_slice.iter().scan(0,
        |state, n| {
            *state += *n;
            Some(*state)
        }
    ).collect();

    let total_sum = accum[accum.len() - 1];
    (input_slice.partition_point(|&x| x <= input_slice[0]) as i64..input_slice[input_slice.len() - 2]).map(
        |mu| {
            let partition_point = input_slice.partition_point(|&x| x < mu);
            mu * (2*partition_point as i64 - input_slice.len() as i64) + total_sum - 2*accum[partition_point - 1]
        }
    ).min().unwrap()
}


#[cfg(feature = "day7")]
pub fn part2(source: String) -> i64 {
    let mut input: Vec<_> = source.split(",").map(
        |n| n.parse::<i64>().unwrap()
    ).collect();
    let input_slice = input.as_mut_slice();
    input_slice.sort_unstable();

    let accum: Vec<_> = input_slice.iter().scan((0, 0),
        |(x, x2), n| {
            *x += *n;
            *x2 += *n * *n;
            Some((*x, *x2))
        }
    ).collect();

    let (total_sum, total_sum2) = accum[accum.len() - 1];
    let total_len = input_slice.len() as i64;
    (input_slice.partition_point(|&x| x <= input_slice[0]) as i64..input_slice[input_slice.len() - 2]).map(
        |mu| {
            let partition_point = input_slice.partition_point(|&x| x < mu);

            let temp = mu*mu*total_len + mu*(2*partition_point as i64 - total_len) + total_sum2 + (1 - 2*mu) * total_sum - 2*accum[partition_point - 1].0;
            temp / 2
        }
    ).min().unwrap()
}

/*
### Part 1
sort -> divide X_low, X_high => sum (mu - X_low)  + sum(X_high - mu) => mu (X_low.len - X_high.len) + sum(X_high) - sum(X_low)

### part 2
arithmetic sum on part 1
X < mu : X_low
(mu - X) (1 + mu - X) / 2 = ( mu + mu^2 - mu X - X - mu X + X^2) / 2 => ((mu + mu^2) * X.len - 2 * mu * sum(X) - sum(X) + sum(X^2))/2
X >= mu : X_high
(X - mu) (1 + X - mu) / 2 = (X + X^2 - X mu - mu - mu X + mu^2) / 2 => sum(X) + sum(X^2) -2mu sum(X) +  X.len *(mu^2 - mu)

(X now means the entire set)
=> mu^2 (X_low + X_high) + mu (X_low.len - X_high_len) + sum(X^2) + sum(X_high) - sum(X_low) - 2mu sum(X) all divided by 2
=> mu^2 (X.len) + mu (2*X_low.len - X.len) + sum(X^2) - 2mu sum(X) + sum(X) - 2 sum(X_low) all divided by 2
=> mu^2 (X.len) + mu (2*X_low.len - X.len) + sum(X^2) + sum(X) (1 - 2mu) - 2 sum(X_low) all divided by 2
*/

/*

With input from reddit solution. Similar part1 performance (85 (mine) vs 81), faster part 2 (85 (mine) vs 63 us)

#[cfg(feature = "day7")]
pub fn part1(source: String) -> i64 {
    let mut input: Vec<_> = source.split(",").map(
        |n| n.parse::<i64>().unwrap()
    ).collect();
    let input_slice = input.as_mut_slice();
    input_slice.sort_unstable();
    let l = input_slice.len();

    let median = if l % 2 == 0 { (input_slice[l / 2 - 1] + input_slice[l / 2])/2 } else { input_slice[ l / 2] };
    input_slice.iter().map(
        |x| (x - median).abs()
    ).sum()
}

#[cfg(feature = "day7")]
pub fn part2(source: String) -> i64 {
    let mut input: Vec<_> = source.split(",").map(
        |n| n.parse::<i64>().unwrap()
    ).collect();
    let input_slice = input.as_mut_slice();
    let l = input_slice.len();

    let mean: i64 = input_slice.iter().sum::<i64>() / input_slice.len() as i64;
    input_slice.iter().map(
        |x| {
            let n = (x - mean).abs();
            n * (n + 1) / 2
        }
    ).sum()
}
*/
