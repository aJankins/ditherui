pub fn sum(nums: &[u8]) -> usize {
    return nums.iter().fold(0, |acc, &e| acc + e as usize)
}

pub fn average(nums: &[u8]) -> f64 {
    return sum(nums) as f64 / nums.len() as f64
}

pub fn map_to_2d(cell_no: u64, xdim: u32) -> (u32, u32) {
    (
        (cell_no % xdim as u64) as u32,
        (cell_no / xdim as u64) as u32
    )
}