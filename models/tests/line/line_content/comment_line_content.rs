use models::core::content::Content;
use models::web_collections::content::comment::CommentContent;

#[test]
fn test_new_comment_line() {
    let a = CommentContent::new("example intro");
    assert_eq!(a.intro, "example intro");
}

#[test]
fn test_get_content() {
    let a = CommentContent::new("example intro");
    assert_eq!(a.get_content(), "# example intro");
}

#[test]
fn test_from_1() {
    let a = CommentContent::from("# example intro");
    assert_eq!(a.intro, "example intro");
}

#[test]
fn test_from_2() {
    let a = CommentContent::from("example intro");
    assert_eq!(a.intro, "[ ? ]");
}

#[test]
fn test_default() {
    let a = CommentContent::default();
    assert_eq!(a.intro, "default intro");
}

#[test]
fn test_display() {
    let a = CommentContent::new("example intro");
    assert_eq!(format!("{}", a), "# example intro");
}
