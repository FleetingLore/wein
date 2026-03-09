use html::LoreHtml;

fn main() {
    use parser::file::file_from_text_to_lore;
    use parser::file::file_from_lore_to_html;
    let example = "
+ tests
  baidu | https://www.baidu.com
  ok
  + 二级领域
    + 三级领域
      aaa
      67890
      + 四级领域
        aaa".to_string();

    let style = r##"<style>
:root {
    --bg: #141516;
    --text: #E6EDF0;
    --text-muted: #c4d9ff;
    --primary: #68c7c7;
    --error: #E67E7E;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    background: var(--bg);
    color: var(--text);
    font-family: sans-serif;
    line-height: 1.6;
    padding: 20px;
}

br {
    height: 1rem;
}

p {
    color: var(--text);
    margin-bottom: 1.2rem;
}

strong {
    color: var(--primary);
    font-size: 1.2rem;
}

a {
    color: var(--text-muted);
    text-decoration: none;
}

a:hover {
    opacity: 0.8;
}
</style>"##;
    
    let lore = file_from_text_to_lore(example);
    let lore_html_lines = file_from_lore_to_html(lore);
    let lore_html = LoreHtml::new(
        "Example".to_string(),
        style.to_string(),
        lore_html_lines,
    );
    let target: String = lore_html.into();
    println!("{}", target);
}
