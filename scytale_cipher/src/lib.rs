pub fn scytale_cipher(message: String, n: u32) -> String {
    let mut rows: Vec<Vec<char>> = Vec::new();
    
    for (i, c) in message.chars().enumerate() {
        if i % n as usize == 0 {
            rows.push(Vec::new());
        }
        rows.last_mut().unwrap().push(c);
        println!("{:?}", rows);
    }
    
    let mut res = String::new();
    let cols = n as usize;
    for col in 0..cols {
        for row in &rows {
            if let Some(&ch) = row.get(col) {
                res.push(ch);
            } else {
                res.push(' ');
            }

        }
    }

    res.trim().to_string()
}