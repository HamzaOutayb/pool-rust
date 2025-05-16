#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res :String = String::new();
    for c in original.chars() {
        if c.is_ascii_lowercase(){
            let sub = (('z' as u8 - c as u8) + 'a' as u8) as char;
            res.push(sub as char)
        } else if c.is_ascii_uppercase() {
            let sub = (('Z' as u8 - c as u8) + 'A' as u8) as char;
            res.push(sub as char)
        } else {
            res.push(c)
        }
    };

    if &res == ciphered {
        return Ok(())
    };

    let c_err = CipherError {
        expected: res,
    };

    return Err(c_err)
}