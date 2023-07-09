use crate::solution::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut start = 0;
        let mut end = s.len() - 1;
        let mut arr: Vec<char> = s.chars().collect();
        let mut temp: char;
        while start < end {
            if is_vowel(arr[start]) && is_vowel(arr[end]) {
                temp = arr[start];
                arr[start] = arr[end];
                arr[end] = temp;
                start += 1;
                end -= 1;
            }

            while start < arr.len() && !is_vowel(arr[start]) {
                start += 1
            }
            while end > 0 && !is_vowel(arr[end]) {
                end -= 1
            }
        }
        return arr.into_iter().collect();
    }

    pub fn reverse_vowels2(mut s: String) -> String {
        let mut bytes = unsafe { s.as_bytes_mut() };

        let mut iter = bytes.iter_mut();
        while let (Some(left), Some(right)) = (
            iter.find(|c| is_vowel2(c)),
            iter.rfind(|c| is_vowel2(c))
        ) {
            std::mem::swap(left, right);
        }
        s
    }
}

fn is_vowel(char: char) -> bool {
    match char {
        'a' |
        'A' |
        'e' |
        'E' |
        'i' |
        'I' |
        'o' |
        'O' |
        'u' |
        'U' => true,
        _ => false
    }
}

fn is_vowel2(c: &u8) -> bool {
    matches!(c.to_ascii_lowercase(), b'a'|b'e'|b'i'|b'o'|b'u')
}