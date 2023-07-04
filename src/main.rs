mod merge_strings;

use crate::merge_strings::Solution as MrStrSolution;

fn main() {
    println!("{}", MrStrSolution {}.merge_alternately(String::from("abc"), String::from("pqr")));
}

