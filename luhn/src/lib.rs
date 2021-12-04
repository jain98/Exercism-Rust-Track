/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    return code
        .chars()
        .filter(|c| !c.is_whitespace())
        .try_rfold((0, 0), |(count, sum), val| {
            val.to_digit(10)
                .map(|n| if count % 2 == 1 { n*2 } else { n })
                .map(|n| if n > 9 { n-9 } else { n })
                .map(|n| (count+1, sum+n))
        })
        .map_or(false, |(count, sum)| {
            sum % 10 == 0 && count > 1
        });
}
