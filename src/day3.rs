// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day3");

#[cfg(not(feature = "day3"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(3, 1)
}

#[cfg(not(feature = "day3"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(3, 2)
}

// v START SOLUTION v
#[cfg(feature = "day3")]
use std::collections::HashSet;

#[cfg(feature = "day3")]
pub fn part1(source: String) -> usize {
    let numbers: Vec<Vec<usize>> = source.split("\r\n").map(|line| line.chars().map(|c| if c == '0' { 0 } else { 1 } ).collect() ).collect();
    let size = numbers.len();
    let mut bits: Vec<usize> = numbers[0].clone();
    let nr_bits = bits.len();
    numbers.into_iter().skip(1).for_each(
        |num| num.into_iter().enumerate().for_each(|(i, b)| bits[i] += b)
    );

    let mut gamma = 0;
    let mut epsilon = 0;
    
    bits.into_iter().enumerate().for_each(
        |(i, count)| {
            let bit = count * 2 / size;
            let offset = nr_bits - i - 1;
            gamma |= bit << offset;
            epsilon |= (!bit & 0b1) << offset;
        }
    );
    return gamma * epsilon;
}

#[cfg(feature = "day3")]
fn get_most_common_bit(numbers: &HashSet<usize>, index: usize, num_len: usize) -> usize {
    let mut tally = 0;
    numbers.iter().for_each(|number| tally += number >> num_len - index - 1 & 0b1);
    tally * 2 / numbers.len()
}

#[cfg(feature = "day3")]
fn filter_out(set: &mut HashSet<usize>, keep_most_common: bool, num_len: usize) {
    let mut i = 0;
    while set.len() > 1 {
        let common_bit = get_most_common_bit(set, i, num_len);
        let keep_one = keep_most_common != (common_bit == 0); 
        set.retain(|number| (number >> num_len - i - 1 & 0b1 == 1) == keep_one);

        i += 1;
    } 
}


#[cfg(feature = "day3")]
pub fn part2(source: String) -> usize {
    let mut num_len = 0;
    let mut oxygen: HashSet<usize> = source.split("\r\n").map(|line| { num_len = line.len(); usize::from_str_radix(line, 2).unwrap()} ).collect();
    let mut co2 = oxygen.clone(); 

    filter_out(&mut oxygen, true, num_len);
    filter_out(&mut co2, false, num_len);
   
    let o2 = oxygen.drain().next().unwrap();
    let co2_num = co2.drain().next().unwrap();
    o2 * co2_num
}
