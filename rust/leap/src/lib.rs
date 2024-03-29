pub fn is_leap_year(year: u64) -> bool {
    match year {
        _ if (year % 4 == 0) && (year % 100 != 0) => true,
        _ if year % 400 == 0 => true,
        _ => false,
    }
}
