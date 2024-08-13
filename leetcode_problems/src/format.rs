impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j {
                    if (nums[i] + nums[j]) == target {
                        return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                    }
                }
            }
        }
        return vec![-1, -1];
    }
}
