pub mod sorts;
pub fn nums(nums: &String) -> Vec<i32> {
    let new_nums: Vec<i32> = nums.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    new_nums
}
