fn max_profit(prices: Vec<i32>) -> i32 {
    let l = prices.len();
    if l <= 0{
        return 0;
    }

    let mut max: i32 = 0;
    let mut min: i32 = prices[0];
    for j in 1..l {
        if prices[j] < min {
            min = prices[j];
        } else {
            max = max.max(prices[j] - min);
        }
    }

    max
}

fn main() {
    println!("max profit: {}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    // println!("max profit: {}", max_profit(vec![7, 2, 6, 1, 3, 2]));
}
