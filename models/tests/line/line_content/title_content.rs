use models::core::content::Content;
use models::web_collections::content::title::TitleContent;

#[test]
fn test_new_title_line() {
    let a = TitleContent::new("example title");
    assert_eq!(a.title, "example title");
}

#[test]
fn test_get_content() {
    let a = TitleContent::new("example title");
    assert_eq!(a.get_content(), "+ example title");
}

#[test]
fn test_from_1() {
    let a = TitleContent::from("+ example title");
    assert_eq!(a.title, "example title");
}

#[test]
fn test_from_2() {
    let a = TitleContent::from("example title");
    assert_eq!(a.title, "[ ? ]");
}

#[test]
fn test_default() {
    let a = TitleContent::default();
    assert_eq!(a.title, "default title");
}

#[test]
fn test_display() {
    let a = TitleContent::new("example title");
    assert_eq!(format!("{}", a), "+ example title");
}
