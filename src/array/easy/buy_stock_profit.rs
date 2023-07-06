/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a different
/// day in the future to sell that stock.
/// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let (mut profit, mut buy) = (0, prices[0]);

    for index in 1..prices.len() {
        profit = i32::max(profit, prices[index] - buy);
        buy = i32::min(buy, prices[index]);
    }

    profit
}

#[cfg(test)]
pub mod buy_stock_tests {
    use super::*;

    #[test]
    fn test_decreasing_order() {
        let prices: Vec<i32> = vec![5, 4, 3, 2];
        let res = max_profit(prices);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_increasing_order() {
        let prices: Vec<i32> = vec![1, 2, 3, 4];
        let res = max_profit(prices);
        assert_eq!(res, 3);
    }
}
