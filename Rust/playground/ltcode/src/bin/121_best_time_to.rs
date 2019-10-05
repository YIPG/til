struct Solution {}

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = std::i32::MAX;
    let mut ans = 0;
    for price in prices.iter() {
      min_price = i32::min(min_price, *price);
      ans = i32::max(ans, price - min_price);
    }
    ans
  }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }
}
