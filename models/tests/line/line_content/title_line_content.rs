use models::core::content::Content;
use models::web_collections::content::title::CommentContent;

#[test]
fn test_new_atom_line() {
    let a = CommentContent::new("example content");
    assert_eq!(a.title, "example content");
}

#[test]
fn test_get_content() {
    let a = CommentContent::new("example content");
    assert_eq!(a.get_content(), "+ example content");
}

#[test]
fn test_from_1() {
    let a = CommentContent::from("+ example title");
    assert_eq!(a.title, "example title");
}

#[test]
fn test_from_2() {
    let a = CommentContent::from("example title");
    assert_eq!(a.title, "[ ? ]");
}

#[test]
fn test_default() {
    let a = CommentContent::default();
    assert_eq!(a.title, "default title");
}

#[test]
fn test_display() {
    let a = CommentContent::new("example title");
    assert_eq!(format!("{}", a), "+ example title");
}
