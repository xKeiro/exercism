pub fn is_valid(code: &str) -> bool {
    let stripped_code = strip_spaces(code);
    if stripped_code.len() <= 1 {
        return false;
    }
    let bcode = stripped_code.as_bytes();
    if !is_all_digits(bcode) {
        return false;
    }
    let luhn_sum = calculate_luhn_sum(bcode);
    luhn_sum % 10 == 0
}

fn strip_spaces(code: &str) -> String {
    code.replace(' ', "")
}

fn is_all_digits(bcode: &[u8]) -> bool {
    bcode.iter().all(|b| b.is_ascii_digit())
}

fn calculate_luhn_sum(bcode: &[u8]) -> usize {
    bcode
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            let digit = (x as char).to_digit(10).unwrap() as u8;
            if i % 2 == 0 {
                return digit;
            }
            match digit {
                0..=4 => digit * 2,
                _ => digit * 2 - 9,
            }
        })
        .sum::<u8>() as usize
}
