/// Masks everything but the last four characters of `s`. Use it before
/// putting any credential derived string in a log line.
///
/// A string of four characters or fewer is masked completely.
pub fn redact(s: &str) -> String {
    let count = s.chars().count();
    if count <= 4 {
        return "*".repeat(count);
    }
    let tail: String = s.chars().skip(count - 4).collect();
    format!("{}{tail}", "*".repeat(count - 4))
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn masks_all_but_the_last_four_characters() {
        let cases = [
            ("", ""),
            ("abc", "***"),
            ("abcd", "****"),
            ("1234567890abcdef", "************cdef"),
            ("密码密码密码", "**密码密码"),
        ];
        for (input, want) in cases {
            assert_eq!(redact(input), want, "redact({input:?})");
        }
    }
}
