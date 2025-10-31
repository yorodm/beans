//! Tag type for categorizing ledger entries.

use crate::error::{BeansError, BeansResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a tag for categorizing ledger entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag {
    name: String,
}

impl Tag {
    /// Creates a new tag with the given name.
    ///
    /// The name is normalized to lowercase and must not be empty.
    pub fn new(name: impl AsRef<str>) -> BeansResult<Self> {
        let name = name.as_ref().trim().to_lowercase();
        
        // Placeholder validation - will be more robust in final implementation
        if name.is_empty() {
            return Err(BeansError::validation(
                "Tag name cannot be empty"
            ));
        }
        
        Ok(Self { name })
    }

    /// Returns the tag name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_new() {
        let tag = Tag::new("groceries").unwrap();
        assert_eq!(tag.name(), "groceries");
    }

    #[test]
    fn test_tag_normalize() {
        let tag = Tag::new("  GROCERIES  ").unwrap();
        assert_eq!(tag.name(), "groceries");
    }
}
