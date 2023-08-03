/// https://leetcode.cn/problems/backspace-string-compare/

pub fn backspace_compare(s: String, t: String) -> bool {
    let builder = |s: String| {
        let mut str = String::new();

        for c in s.chars() {
            if c == '#' {
                str.pop();
            } else {
                str.push(c);
            }
        }

        str
    };

    builder(s) == builder(t)
}

#[cfg(test)]
mod tests {
    #[test]
    fn backspace_compare_it_works() {
        use super::backspace_compare;
        assert!(backspace_compare("ab#c".to_string(), "ad#c".to_string()));
        assert!(backspace_compare("ab##".to_string(), "c#d#".to_string()));
        assert!(backspace_compare("a##c".to_string(), "#a#c".to_string()));
        assert!(!backspace_compare("a#c".to_string(), "b".to_string()));
        assert_eq!(
            backspace_compare("bxj##tw".to_string(), "bxj###tw".to_string()),
            false
        );
    }
}
