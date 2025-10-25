use core::fmt;
use std::{collections::HashMap, error::Error};

// Template
pub struct DomainErrorTemplate {
    pub code: &'static str,
    pub message: &'static str,
    pub attributes: Option<HashMap<String, String>>,
}

impl DomainErrorTemplate {
    pub const fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            code,
            message,
            attributes: Option::None,
        }
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> DomainErrorTemplate {
        let mut attributes = self.attributes.unwrap_or(HashMap::new());
        attributes.insert(key.into(), value.into());
        self.attributes = Option::Some(attributes);
        self
    }

    pub fn build(self) -> DomainError {
        DomainError {
            code: self.code,
            message: self.message,
            attributes: self.attributes.unwrap_or(HashMap::new()),
        }
    }
}

// Domain error definiton
#[derive(Debug, Clone)]
pub struct DomainError {
    pub code: &'static str,
    pub message: &'static str,
    pub attributes: HashMap<String, String>,
}

impl DomainError {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            code,
            message,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} {:?}", self.code, self.message, self.attributes)
    }
}

impl Error for DomainError {}
