pub fn sum(nums: &[u8]) -> usize {
    return nums.iter().fold(0, |acc, &e| acc + e as usize)
}

pub fn average(nums: &[u8]) -> f64 {
    return sum(nums) as f64 / nums.len() as f64
}

pub fn sq_u8(num: &u8) -> u64 {
    (*num as u64).pow(2)
}