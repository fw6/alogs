pub fn valid_parentheses(str: String) -> bool {
    if str.len() % 2 == 1 {
        return false;
    }

    let mut stack = vec![];

    for c in str.into_bytes() {
        match c {
            b'(' | b'[' | b'{' => {
                stack.push(c + (1 << (c & 1)));
            }
            _ => {
                if Some(c) != stack.pop() {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}
