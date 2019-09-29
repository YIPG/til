struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums.iter() {
      match set.get(&num) {
        None => {set.insert(num);},
        Some(_) => return true
      }
    }

    false
  }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
    }
}
