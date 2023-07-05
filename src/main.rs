use crate::candies::Solution as CandiesSolution;
use crate::gcd_of_strings::Solution as GcdSolution;
use crate::merge_strings::Solution as MrStrSolution;

mod merge_strings;
mod gcd_of_strings;
mod candies;

fn main() {
    println!("{}", MrStrSolution {}.merge_alternately(String::from("abc"), String::from("pqr")));
    println!("{}", GcdSolution {}.gcd_of_strings(String::from("abc"), String::from("abcabc")));
    let candies_map = CandiesSolution {}.kids_with_candies(vec![12, 1, 12], 4);
    print!("[");
    for i in candies_map {
        print!("{}, ", i);
    }
    print!("]")
}

