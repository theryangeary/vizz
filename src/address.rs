#[derive(Debug, Clone, Default)]
/// A string containing a hex value starting with `0x`
pub struct Address(String);

impl Address {
    /// Create a new address struct with the address referenced by the input t
    pub fn new<T>(t: &T) -> Self {
        let ptr: *const _ = t;
        Self(format!("{:?}", ptr))
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
