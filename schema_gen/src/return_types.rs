pub fn actual_type(t: &str) -> String {
    let ty = t.split(':').last().map(|r| format!("Option<{}>", r));

    ty.unwrap_or_else(|| "()".to_string())
}

/*



pub fn fn_name(&self) -> String {
    self.type_name().to_case(Case::Snake)
}*/
