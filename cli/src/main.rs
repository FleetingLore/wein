use std::fs;
use std::path::Path;
use html::LoreHtml;
use parser::file::file_from_text_to_lore;
use parser::file::file_from_lore_to_html;

fn main() {
    let example_lore = Path::new("test/lore/local.lore");
    let example_lore = fs::read_to_string(example_lore).unwrap().to_string();
    let example_lore = file_from_text_to_lore(example_lore);
    let example_lore = file_from_lore_to_html(example_lore);

    let style = Path::new("test/css/style.css");
    let style = fs::read_to_string(style).unwrap().to_string();

    let target: String = LoreHtml::new(
        "Example".to_string(),
        style.to_string(),
        example_lore
    ).into();

    fs::write("test/output/example.html", target).expect("File writing error");
}
