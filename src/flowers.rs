use crate::solution::Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut plots = n;
        for i in 0..(flowerbed.len()) {
            match i {
                0 => {
                    if flowerbed[i] == 0 && (i + 1 == flowerbed.len() || flowerbed[i + 1] == 0)
                    {
                        plots -= 1;
                        flowerbed[i] = 1;
                    }
                }
                _ => {
                    if flowerbed[i] == 0
                        && flowerbed[i - 1] == 0
                        && (i + 1 == flowerbed.len() || flowerbed[i + 1] == 0)
                    {
                        plots -= 1;
                        flowerbed[i] = 1;
                    }
                }
            }
            if plots <= 0 { break; }
        }
        return plots <= 0;
    }
}