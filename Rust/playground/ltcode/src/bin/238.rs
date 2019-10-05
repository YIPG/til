struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 { return vec![] }
        let mut res = vec![1; nums.len()];
        let mut n = 1;
        for i in (0..nums.len()-1).rev() {
            n *= nums[i+1];
            res[i] = n;
        }
        n = 1;
        for i in 1..nums.len() {
            n *= nums[i-1];
            res[i] *= n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    }
}
