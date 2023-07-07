use crate::solution::Solution;

mod merge_strings;
mod gcd_of_strings;
mod candies;
mod flowers;
mod solution;
mod reverse_strings;

fn main() {
    println!("{}", Solution::merge_alternately(String::from("abc"), String::from("pqr")));
    println!("{}", Solution::gcd_of_strings(String::from("abc"), String::from("abcabc")));
    test_candies();
    test_candies2();
    println!("{}", Solution::can_place_flowers(vec![0, 1, 0], 1));
    println!("{}", Solution::reverse_vowels("Hi leetcode".to_owned()))
}

fn test_candies() {
    let candies_map = Solution::kids_with_candies(vec![12, 1, 12], 4);
    print!("[");
    for i in candies_map {
        print!("{}, ", i);
    }
    print!("]");
}

fn test_candies2() {
    let candies_map = Solution::kids_with_candies2(vec![12, 1, 12], 4);
    print!("[");
    for i in candies_map {
        print!("{}, ", i);
    }
    print!("]");
}

