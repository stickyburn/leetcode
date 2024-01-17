mod two_sum_01;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!(
        "Index of two numbers that add to 9: {:?}",
        two_sum_01::SumOfTwo::sum(nums, target)
    );
}
