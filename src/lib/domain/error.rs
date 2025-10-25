use core::fmt;
use std::{collections::HashMap, error::Error};

// Template
pub struct DomainErrorTemplate {
    pub code: &'static str,
    pub message: &'static str,
}

impl DomainErrorTemplate {
    pub const fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            code,
            message,
        }
    }

    pub fn with_attr(self, key: impl Into<String>, value: impl Into<String>) -> DomainError {
        let mut attributes = HashMap::new();
        attributes.insert(key.into(), value.into());
        DomainError {
            code: self.code,
            message: self.message,
            attributes: attributes,
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
