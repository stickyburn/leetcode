use std::collections::HashMap;

struct SumOfTwo;

impl SumOfTwo {
    fn sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                map.insert(num, i as i32);
            }
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 3];
    let target = 10;
    println!(
        "Index of two numbers that add to 9: {:?}",
        SumOfTwo::sum(nums, target)
    );
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(SumOfTwo::sum(nums, target), vec![0, 1]);
}
