pub fn is_leap_year(year: u64) -> bool {
    match (year%4, year%100, year%400) {
        (0, 0, 0) => true,
        (0, 1..=99, _) => true,
        _ => false
    }
}
