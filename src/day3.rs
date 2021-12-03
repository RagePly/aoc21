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
    let mut nr_bits = 0;
    let numbers: Vec<usize> = source.split("\r\n").map(|line| { nr_bits = line.len(); usize::from_str_radix(line, 2).unwrap() } ).collect();
    let size = numbers.len();

    let mut bit_tally: Vec<usize> = Vec::new();
    bit_tally.resize(nr_bits, 0);

    numbers.into_iter().for_each(
        |num| (0..nr_bits).for_each(|i| bit_tally[i] += 0b1 & num >> nr_bits - i - 1)
    );

    let mut gamma = 0;
    let mut epsilon = 0;
    
    bit_tally.into_iter().enumerate().for_each(
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
