pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

    // Base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if source_chars[i - 1] == target_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + min(
                    dp[i - 1][j],     // Deletion
                    dp[i][j - 1],     // Insertion
                    dp[i - 1][j - 1], // replacement
                );
            }
        }
    }

    dp[m][n]
}

fn min(a: usize, b: usize, c: usize) -> usize {
    a.min(b.min(c))
}


pub fn expected_variable(s1: &str, s2: &str)-> Option<String> {
    let s1 = s1.to_string().to_lowercase();
    let s2 = s2.to_string().to_lowercase();
     if !is_snake_case(&s2) && !is_camel_case(&s2) {
        return None;
    } 
        let distance = edit_distance(&s1, &s2);
        let len = s2.len().max(1);
        let calc = (100.*(1.-(distance as f64/len as f64))).round();
        
        if calc > 50. {
            return Some(format!("{}%", calc))
        }
        None
}
fn is_snake_case(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        && !s.starts_with('_')
        && !s.ends_with('_')
        && !s.contains("__")
}

fn is_camel_case(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        first.is_ascii_lowercase()
            && !s.contains('_')
            && s.chars().any(|c| c.is_ascii_uppercase())
    } else {
        false
    }
}
