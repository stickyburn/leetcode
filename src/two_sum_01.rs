use std::collections::HashMap;

pub struct SumOfTwo;

impl SumOfTwo {
    pub fn sum(nums: Vec<u32>, target: u32) -> Vec<u32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i as u32];
            } else {
                map.insert(num, i as u32);
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(SumOfTwo::sum(nums, target), vec![0, 1]);
}
