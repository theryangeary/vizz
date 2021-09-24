use crate::constants::*;

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

pub fn render_port(hex_address_string: &str, suffix: &str) -> String {
    format!("{}{}{}", hex_address_string, PORT_SEPARATOR, suffix)
}

pub fn render_label_port(hex_address_string: &str) -> String {
    render_port(hex_address_string, LABEL)
}

pub fn render_type_port(hex_address_string: &str) -> String {
    render_port(hex_address_string, TYPE)
}

pub fn render_address_port(hex_address_string: &str) -> String {
    render_port(hex_address_string, ADDRESS)
}

pub fn render_value_port(hex_address_string: &str) -> String {
    render_port(hex_address_string, VALUE)
}

pub fn render_associated_data_port(hex_address_string: &str) -> String {
    render_port(hex_address_string, ASSOCIATED_DATA)
}

pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}
