pub enum Accept {
    ApplicationJson,
}

impl Accept {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ApplicationJson => "application/json",
        }
    }
}
