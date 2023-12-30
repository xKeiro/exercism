pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .try_fold(0_u32, |acc, digit| acc.checked_add(digit.pow(num_digits)))
        .is_some_and(|sum| sum == num)
}
