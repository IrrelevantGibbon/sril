pub(crate) fn safe_extract(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];

    (remainder, extracted)
}

pub(crate) fn extract(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_message: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = safe_extract(accept, s);

    if extracted.is_empty() {
        Err(error_message)
    } else {
        Ok((remainder, extracted))
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    extract(|c| c.is_ascii_digit(), s, "Expected digits".to_string())
}

const WHITESPACE: &[char] = &[' ', '\n', '\t'];

pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    safe_extract(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_required_whitespaces(s: &str) -> Result<(&str, &str), String> {
    extract(|c| WHITESPACE.contains(&c), s, "Expected space".to_string())
}

pub(crate) fn extract_identifier(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        extract(
            |c| c.is_alphanumeric(),
            s,
            "Expected identifier".to_string(),
        )
    } else {
        Err("Identifier not found".to_string())
    }
}

pub(crate) fn extract_tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("Expected digits".to_string()));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), Err("Expected digits".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespaces("    1"), ("1", "    "));
    }

    #[test]
    fn do_not_extract_spaces_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_required_whitespaces("blah"),
            Err("Expected space".to_string()),
        );
    }

    #[test]
    fn extract_newlines_or_spaces() {
        assert_eq!(extract_whitespaces(" \n   \n\nabc"), ("abc", " \n   \n\n"));
    }

    #[test]
    fn extract_alphanumeric_identifier() {
        assert_eq!(extract_identifier("abcde()"), Ok(("()", "abcde")))
    }

    #[test]
    fn tag_word() {
        assert_eq!(extract_tag("let", "let a"), Ok(" a"));
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_identifier("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_identifier("foobar1()"), Ok(("()", "foobar1")));
    }
}
