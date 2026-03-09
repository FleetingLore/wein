use models::domain::file::File;

fn main() {
    use parser::file::file_from_text_to_lore;
    use parser::file::file_from_lore_to_html;
    let example = "
+ tests
  baidu | https://www.baidu.com
  ok
  + 二级领域
    + 三级领域
      + 四级领域
        aaa".to_string();
    
    let lore = file_from_text_to_lore(example);
    let html = file_from_lore_to_html(
        File {
            name: "Example".to_string(),
            lines: lore
        });
    println!("{}", html);
}
