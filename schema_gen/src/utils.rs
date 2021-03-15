const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

pub fn escape(s: &str) -> String {
    let s = s.split(":").last().unwrap_or(s);

    let idx = s.chars().next().and_then(|c| c.to_digit(10));

    if let Some(i) = idx {
        let mut r = DIGITS[i as usize].to_owned();
        r.push_str(&s[1..]);

        return r.to_string();
    }

    s.to_string()
}
