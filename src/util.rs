pub fn html_encode(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// Render a table, unless there would be no rows in that table, because that is a syntax error in
/// DOT language
pub fn render_table(table_rows: impl Iterator<Item = String>) -> String {
    let table_rows_string = table_rows.fold(String::new(), |acc, s| acc + &s);
    match table_rows_string.len() {
        0 => String::new(),
        _ => format!(
            r#"<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">{}</TABLE>"#,
            table_rows_string
        ),
    }
}

pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}
