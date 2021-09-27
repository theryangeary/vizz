use crate::constants::{ADDRESS, ASSOCIATED_DATA, LABEL, PORT_SEPARATOR, TYPE, VALUE};

#[derive(Debug, Clone, Default)]
/// A string containing a hex value starting with `0x`
pub struct Address(String);

impl Address {
    /// Create a new address struct with the address referenced by the input t
    pub fn new<T>(t: &T) -> Self {
        let ptr: *const _ = t;
        Self(format!("{:?}", ptr))
    }

    fn render_port(&self, suffix: &str) -> String {
        format!("{}{}{}", self, PORT_SEPARATOR, suffix)
    }

    /// Render table data (<TD>) label port name
    pub fn render_label_port(&self) -> String {
        self.render_port(LABEL)
    }

    /// Render table data (<TD>) type port name
    pub fn render_type_port(&self) -> String {
        self.render_port(TYPE)
    }

    /// Render table data (<TD>) address port name
    pub fn render_address_port(&self) -> String {
        self.render_port(ADDRESS)
    }

    /// Render table data (<TD>) value port name
    pub fn render_value_port(&self) -> String {
        self.render_port(VALUE)
    }

    /// Render table data (<TD>) associated_data port name
    pub fn render_associated_data_port(&self) -> String {
        self.render_port(ASSOCIATED_DATA)
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

// allow From<&str> for easy test writing
#[cfg(test)]
impl From<&str> for Address {
    fn from(address_string: &str) -> Self {
        Self(address_string.into())
    }
}
