/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.replace(' ', "");
    if stripped_code.len() <= 1 {
        return false;
    }
    let bcode = stripped_code.as_bytes();
    if !bcode.iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let sum: usize = bcode
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            let digit = (x as char).to_digit(10).unwrap() as u8;
            if i % 2 == 0 {
                return digit;
            }
            match digit {
                (0..=4) => digit * 2,
                _ => digit * 2 - 9,
            }
        })
        .fold(0, |acc, x| acc + x as usize);
    if sum % 10 != 0 {
        return false;
    }
    true
}
