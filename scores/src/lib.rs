pub fn score(s: &str) -> u64 {
    let mut sum: u64 = 0;
    for c in s.chars() {
        let sub = match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T'| 'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'D' | 'G'|'d' | 'g' => 2,
            'B' | 'C' | 'M' | 'P'|'b' | 'c' | 'm' | 'p' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y'| 'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'K'| 'k' => 5,
            'J' | 'X'| 'j' | 'x' => 8,
            'Q' | 'Z'| 'q' | 'z' => 10,
            _ => 0,
        };
        sum += sub;
    }
    sum
}