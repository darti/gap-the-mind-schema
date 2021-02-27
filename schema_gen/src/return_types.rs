pub fn actual_type(t: &str) -> String {
    let ty = t.split(":").last().map(|r| format!("Option<{}>", r));

    ty.unwrap_or("()".to_string())
}

/*
pub fn doc(&self) -> String {
    match &self.comment {
        Some(t) => t.to_string(),
        None => "".to_string(),
    }
}

pub fn type_name(&self) -> String {
    let l = self.label.to_string();

    let idx = l.chars().nth(0).and_then(|c| c.to_digit(10));

    if let Some(i) = idx {
        let mut r = DIGITS[i as usize].to_owned();
        r.push_str(&l[1..]);

        return r.to_string();
    }

    l
}

pub fn fn_name(&self) -> String {
    self.type_name().to_case(Case::Snake)
}*/
