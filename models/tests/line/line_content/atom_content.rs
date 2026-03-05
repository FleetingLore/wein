use models::core::content::Content;
use models::web_collections::content::atom::AtomContent;

#[test]
fn test_new_atom_line() {
    let a = AtomContent::new("example content");
    assert_eq!(a.content, "example content");
}

#[test]
fn test_get_content() {
    let a = AtomContent::new("example content");
    assert_eq!(a.get_content(), "example content");
}

#[test]
fn test_from() {
    let a = AtomContent::from("example content");
    assert_eq!(a.content, "example content");
}

#[test]
fn test_default() {
    let a = AtomContent::default();
    assert_eq!(a.content, "default content");
}

#[test]
fn test_display() {
    let a = AtomContent::new("example content");
    assert_eq!(format!("{}", a), "example content");
}
