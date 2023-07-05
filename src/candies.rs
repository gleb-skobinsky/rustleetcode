pub struct Solution {}

impl Solution {
    pub fn kids_with_candies(&self, candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = 0;
        let max_candies = candies.iter().max();
        if !max_candies.is_none() {
            max = max_candies.unwrap().to_owned();
        }
        return candies.iter().map(|it| max <= it + extra_candies).collect();
    }

    pub fn kids_with_candies2(&self, candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = candies.iter().max().unwrap();
        candies.iter().map(|i| i + extra_candies >= *max_candies).collect()
    }
}