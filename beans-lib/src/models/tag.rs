//! Tag type for categorizing ledger entries.

use crate::error::{BeansError, BeansResult};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Maximum length for a tag name
const MAX_TAG_LENGTH: usize = 50;

/// Represents a tag for categorizing ledger entries.
///
/// Tags are used to categorize and filter ledger entries. They are normalized
/// to lowercase and trimmed of whitespace to ensure consistent matching.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag {
    name: String,
}

impl Tag {
    /// Creates a new tag with the given name.
    ///
    /// The name is normalized to lowercase, trimmed of whitespace, and must:
    /// - Not be empty
    /// - Not exceed 50 characters
    /// - Not contain special characters except for hyphens and underscores
    ///
    /// # Examples
    ///
    /// ```
    /// use beans_lib::models::Tag;
    ///
    /// let tag = Tag::new("groceries").unwrap();
    /// assert_eq!(tag.name(), "groceries");
    ///
    /// // Tags are normalized
    /// let tag = Tag::new("  GROCERIES  ").unwrap();
    /// assert_eq!(tag.name(), "groceries");
    ///
    /// // Invalid tags
    /// assert!(Tag::new("").is_err()); // Empty
    /// assert!(Tag::new("a tag with spaces").is_err()); // Contains spaces
    /// assert!(Tag::new("tag#with#special#chars").is_err()); // Special characters
    /// ```
    pub fn new(name: impl AsRef<str>) -> BeansResult<Self> {
        let name = name.as_ref().trim().to_lowercase();
        
        // Check if empty
        if name.is_empty() {
            return Err(BeansError::validation(
                "Tag name cannot be empty"
            ));
        }
        
        // Check length
        if name.len() > MAX_TAG_LENGTH {
            return Err(BeansError::validation(
                format!("Tag name cannot exceed {} characters", MAX_TAG_LENGTH)
            ));
        }
        
        // Check for invalid characters (allow alphanumeric, hyphens, and underscores)
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(BeansError::validation(
                "Tag name can only contain letters, numbers, hyphens, and underscores"
            ));
        }
        
        // Check for spaces
        if name.contains(' ') {
            return Err(BeansError::validation(
                "Tag name cannot contain spaces"
            ));
        }
        
        Ok(Self { name })
    }

    /// Returns the tag name.
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Creates a tag from a string without validation.
    ///
    /// This is intended for internal use only, such as when loading tags from a database
    /// where they have already been validated.
    ///
    /// # Safety
    ///
    /// This method bypasses validation and should only be used when the tag name
    /// is known to be valid.
    pub(crate) fn from_raw(name: String) -> Self {
        Self { name }
    }
    
    /// Attempts to create multiple tags from a comma-separated string.
    ///
    /// Returns a vector of valid tags. If any tag is invalid, returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use beans_lib::models::Tag;
    ///
    /// let tags = Tag::from_comma_separated("groceries,food,household").unwrap();
    /// assert_eq!(tags.len(), 3);
    /// assert_eq!(tags[0].name(), "groceries");
    /// ```
    pub fn from_comma_separated(tags_str: impl AsRef<str>) -> BeansResult<Vec<Self>> {
        let tags_str = tags_str.as_ref().trim();
        if tags_str.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut tags = Vec::new();
        for tag_str in tags_str.split(',') {
            let tag = Self::new(tag_str)?;
            tags.push(tag);
        }
        
        Ok(tags)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Tag {
    type Err = BeansError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for Tag {
    type Error = BeansError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Self::new(name)
    }
}

impl TryFrom<String> for Tag {
    type Error = BeansError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name)
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
    
    #[test]
    fn test_tag_validation() {
        // Empty tag
        assert!(Tag::new("").is_err());
        assert!(Tag::new("   ").is_err());
        
        // Too long
        let long_name = "a".repeat(MAX_TAG_LENGTH + 1);
        assert!(Tag::new(long_name).is_err());
        
        // Invalid characters
        assert!(Tag::new("tag with spaces").is_err());
        assert!(Tag::new("tag#with#hash").is_err());
        assert!(Tag::new("tag@with@at").is_err());
        assert!(Tag::new("tag/with/slash").is_err());
        
        // Valid characters
        assert!(Tag::new("tag-with-hyphens").is_ok());
        assert!(Tag::new("tag_with_underscores").is_ok());
        assert!(Tag::new("tag123with456numbers").is_ok());
        assert!(Tag::new("123numeric").is_ok());
    }
    
    #[test]
    fn test_tag_from_str() {
        let tag: Tag = "groceries".parse().unwrap();
        assert_eq!(tag.name(), "groceries");
        
        let result: Result<Tag, _> = "invalid tag".parse();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_tag_try_from() {
        let tag: Tag = "groceries".try_into().unwrap();
        assert_eq!(tag.name(), "groceries");
        
        let tag: Tag = String::from("food").try_into().unwrap();
        assert_eq!(tag.name(), "food");
        
        let result: Result<Tag, _> = "invalid tag".try_into();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_tag_display() {
        let tag = Tag::new("groceries").unwrap();
        assert_eq!(format!("{}", tag), "groceries");
    }
    
    #[test]
    fn test_tag_from_comma_separated() {
        // Single tag
        let tags = Tag::from_comma_separated("groceries").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name(), "groceries");
        
        // Multiple tags
        let tags = Tag::from_comma_separated("groceries,food,household").unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].name(), "groceries");
        assert_eq!(tags[1].name(), "food");
        assert_eq!(tags[2].name(), "household");
        
        // Whitespace handling
        let tags = Tag::from_comma_separated(" groceries , FOOD ,household ").unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].name(), "groceries");
        assert_eq!(tags[1].name(), "food");
        assert_eq!(tags[2].name(), "household");
        
        // Empty string
        let tags = Tag::from_comma_separated("").unwrap();
        assert_eq!(tags.len(), 0);
        
        // Invalid tag
        let result = Tag::from_comma_separated("groceries,invalid tag,household");
        assert!(result.is_err());
    }
}
