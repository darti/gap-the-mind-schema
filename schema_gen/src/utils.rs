const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

const RESERVED_KEYWORDS: [&str; 52] = [
    "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure",
    "ref", "return", "Self", "self", "sizeof", "static", "struct", "super", "trait", "true",
    "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

pub fn escape(s: &str) -> String {
    let s = s.split(":").last().unwrap_or(s);

    let idx = s.chars().next().and_then(|c| c.to_digit(10));

    if let Some(i) = idx {
        let mut r = DIGITS[i as usize].to_owned();
        r.push_str(&s[1..]);

        return r.to_string();
    }

    if RESERVED_KEYWORDS.contains(&s) {
        format!("r#{}", s)
    } else {
        s.to_string()
    }
}
