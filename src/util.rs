use crate::address::Address;
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

pub fn render_port(address: &Address, suffix: &str) -> String {
    format!("{}{}{}", address, PORT_SEPARATOR, suffix)
}

pub fn render_label_port(address: &Address) -> String {
    render_port(address, LABEL)
}

pub fn render_type_port(address: &Address) -> String {
    render_port(address, TYPE)
}

pub fn render_address_port(address: &Address) -> String {
    render_port(address, ADDRESS)
}

pub fn render_value_port(address: &Address) -> String {
    render_port(address, VALUE)
}

pub fn render_associated_data_port(address: &Address) -> String {
    render_port(address, ASSOCIATED_DATA)
}

pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}
