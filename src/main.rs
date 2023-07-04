mod merge_strings;
mod gcd_of_strings;

use crate::merge_strings::Solution as MrStrSolution;
use crate::gcd_of_strings::Solution as GcdSolution;

fn main() {
    println!("{}", MrStrSolution {}.merge_alternately(String::from("abc"), String::from("pqr")));
    println!("{}", GcdSolution {}.gcd_of_strings(String::from("abc"), String::from("abcabc")));
}

