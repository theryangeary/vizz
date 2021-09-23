pub fn html_encode(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn render_table(table_rows: impl Iterator<Item = String>) -> String {
    format!(
        r#"<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">{}</TABLE>"#,
        table_rows.fold(String::new(), |acc, s| acc + &s)
    )
}

pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}
