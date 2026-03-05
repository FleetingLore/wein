use models::core::content::Content;
use models::web_collections::content::link::LinkContent;

#[test]
fn test_new_atom_line() {
    let a = LinkContent::new("example info", "example url");
    assert_eq!(a.info, "example info");
    assert_eq!(a.url, "example url");
}

#[test]
fn test_get_content() {
    let a = LinkContent::new("example info", "example url");
    assert_eq!(a.get_content(), "example info = example url");
}

#[test]
fn test_from_1() {
    let a = LinkContent::from("example info = example url");
    assert_eq!(a.info, "example info");
    assert_eq!(a.url, "example url");
}

#[test]
fn test_from_2() {
    let a = LinkContent::from("example info");
    assert_eq!(a.info, "example info");
    assert_eq!(a.url, "[ ? ]");
}

#[test]
fn test_default() {
    let a = LinkContent::default();
    assert_eq!(a.info, "default info");
    assert_eq!(a.url, "default url");
}

#[test]
fn test_display() {
    let a = LinkContent::new("example info", "example url");
    assert_eq!(format!("{}", a), "example info = example url");
}
