pub fn kmp_match<'a, 'b>(text: &'a str, pat: &'b str) -> Option<usize> {
    if pat.len() == 0 {
        return Some(0)
    }

    let next = generate_next(pat);
    let mut i = 0;
    let mut j = 0;
    let text_bytes = text.as_bytes();
    let pat_bytes = pat.as_bytes();
    while i <= text.len() - pat.len() {
        while j < pat.len() && text_bytes[i] == pat_bytes[j] {
            j += 1;
            i += 1;
        }
        if j == pat.len() {
            return Some(i - j);
        } else if j > 0 {
            j = next[j - 1];
        } else {
            i += 1;
        }
    }
    None
}

fn generate_next(pat: &str) -> Vec<usize> {
    let mut next = vec![0usize; pat.len()];
    let mut i = 1;
    let mut j;
    let bytes = pat.as_bytes();
    while i < pat.len() {
        j = next[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = next[(j - 1) as usize];
        }
        next[i] = if bytes[i] == bytes[j] { j + 1 } else { 0 };
        i += 1;
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let text = "hello, world";
        assert_eq!(kmp_match(text, "hello"), Some(0));
        assert_eq!(kmp_match(text, "world"), Some(7));
        assert_eq!(kmp_match(text, "hallo"), None);

        assert_eq!(kmp_match(text, ""), Some(0));

        assert_eq!(kmp_match("abcdababcabcddef", "abcabcd"), Some(6));

    }

    #[test]
    fn test_generate_next() {
        let pat = "hello, world";
        assert_eq!(generate_next(pat), vec![0; pat.len()]);

        let pat = "abaab";
        assert_eq!(generate_next(pat), vec![0, 0, 1, 1, 2]);

        let pat = "abcabcd";
        assert_eq!(generate_next(pat), vec![0, 0, 0, 1, 2, 3, 0]);
    }
}
