pub struct Solution {}

impl Solution {
    pub fn merge_alternately(&self, word1: String, word2: String) -> String {
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        let mut result = String::with_capacity(word1.len() + word2.len());

        loop {
            match (iter1.next(), iter2.next()) {
                (Some(_m @ a), Some(_n @ b)) => {
                    result.push(a);
                    result.push(b);
                }
                (None, Some(_n @ b)) => result.push(b),
                (Some(_m @ a), None) => result.push(a),
                _ => break,
            }
        }

        result
    }
}