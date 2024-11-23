use std::io;

pub fn longest_common_subsequence(text1: String, text2: String) -> String {
    let input1_len = text1.len();
    let input2_len = text2.len();

    let mut dp = vec![vec![0; input2_len + 1]; input1_len + 1];
    let mut backtrack = vec![vec![0; input2_len + 1]; input1_len + 1]; // Track the optimal path

    for iter1 in 1..=input1_len {
        for iter2 in 1..=input2_len {
            if text1.chars().nth(iter1 - 1).unwrap() == text2.chars().nth(iter2 - 1).unwrap() {
                dp[iter1][iter2] = dp[iter1 - 1][iter2 - 1] + 1;
                backtrack[iter1][iter2] = 3; // Diagonal: Match
            } else if dp[iter1 - 1][iter2] >= dp[iter1][iter2 - 1] {
                dp[iter1][iter2] = dp[iter1 - 1][iter2];
                backtrack[iter1][iter2] = 1; // Up: Skip a character in text1
            } else {
                dp[iter1][iter2] = dp[iter1][iter2 - 1];
                backtrack[iter1][iter2] = 2; // Left: Skip a character in text2
            }
        }
    }

    let mut iter1 = input1_len;
    let mut iter2 = input2_len;
    let mut lcs_result = String::new();

    while iter1 > 0 && iter2 > 0 {
        match backtrack[iter1][iter2] {
            3 => {
                lcs_result.push(text1.chars().nth(iter1 - 1).unwrap());
                iter1 -= 1;
                iter2 -= 1;
            }
            1 => {
                iter1 -= 1;
            }
            2 => {
                iter2 -= 1;
            }
            _ => unreachable!(),
        }
    }

    lcs_result.chars().rev().collect()
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input1 :/");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input2 :/");

    let lcs_result = longest_common_subsequence(input1, input2);
    println!("The longest common subsequence is: {}", lcs_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        // Complete match
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "abcde".to_string()),
            "abcde".to_string()
        );
        // Partial match
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "aced".to_string()),
            "ace".to_string()
        );
        // No match
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "wxyz".to_string()),
            "".to_string()
        );
    }
}
