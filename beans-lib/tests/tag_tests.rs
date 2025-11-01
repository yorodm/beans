use beans_lib::models::Tag;
use std::str::FromStr;

#[test]
fn test_tag_new() {
    let tag = Tag::new("groceries").unwrap();
    assert_eq!(tag.name(), "groceries");
}

#[test]
fn test_tag_normalize() {
    let tag = Tag::new("GROCERIES").unwrap();
    assert_eq!(tag.name(), "groceries");

    let tag = Tag::new(" food ").unwrap();
    assert_eq!(tag.name(), "food");
}

#[test]
fn test_tag_validation() {
    // Empty tag
    assert!(Tag::new("").is_err());
    assert!(Tag::new("   ").is_err());

    // Too long tag
    let long_tag = "a".repeat(51);
    assert!(Tag::new(long_tag).is_err());

    // Invalid characters
    assert!(Tag::new("groceries!").is_err());
    assert!(Tag::new("food&drinks").is_err());
    assert!(Tag::new("tag with spaces").is_err());

    // Valid characters
    assert!(Tag::new("groceries").is_ok());
    assert!(Tag::new("food123").is_ok());
    assert!(Tag::new("food-and-drinks").is_ok());
    assert!(Tag::new("food_and_drinks").is_ok());
}

#[test]
fn test_tag_from_str() {
    let tag = Tag::from_str("groceries").unwrap();
    assert_eq!(tag.name(), "groceries");

    let tag = Tag::from_str("GROCERIES").unwrap();
    assert_eq!(tag.name(), "groceries");

    assert!(Tag::from_str("").is_err());
    assert!(Tag::from_str("invalid!").is_err());
}

#[test]
fn test_tag_try_from() {
    let tag: Tag = "groceries".try_into().unwrap();
    assert_eq!(tag.name(), "groceries");

    let tag: Tag = String::from("GROCERIES").try_into().unwrap();
    assert_eq!(tag.name(), "groceries");

    let result: Result<Tag, _> = "".try_into();
    assert!(result.is_err());
}

#[test]
fn test_tag_display() {
    let tag = Tag::new("groceries").unwrap();
    assert_eq!(format!("{}", tag), "groceries");
}

#[test]
fn test_tag_from_comma_separated() {
    let tags = Tag::from_comma_separated("groceries,food,household").unwrap();
    assert_eq!(tags.len(), 3);

    let names: Vec<_> = tags.iter().map(|t| t.name()).collect();
    assert!(names.contains(&"groceries"));
    assert!(names.contains(&"food"));
    assert!(names.contains(&"household"));

    // Test with spaces
    let tags = Tag::from_comma_separated("groceries, food, household").unwrap();
    assert_eq!(tags.len(), 3);

    // Test with empty parts - empty parts are skipped
    let tags = Tag::from_comma_separated("groceries,food").unwrap();
    assert_eq!(tags.len(), 2);

    // Test with invalid tag
    assert!(Tag::from_comma_separated("groceries,invalid!").is_err());
}
