pub struct Solution {}

impl Solution {
    pub fn gcd_of_strings(&self, str1: String, str2: String) -> String {
        let sum_str1 = format!("{}{}", str1, str2);
        let sum_str2 = format!("{}{}", str2, str1);
        if sum_str1 != sum_str2 {
            return "".to_owned();
        }
        let mut length_of_str1 = str1.len();
        let mut length_of_str2 = str2.len();
        while length_of_str1 != length_of_str2 {
            if length_of_str1 > length_of_str2 {
                length_of_str1 -= length_of_str2
            } else {
                length_of_str2 -= length_of_str1
            }
        }
        return str1[0..length_of_str1].to_owned();
    }
}