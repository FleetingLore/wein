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
    
    let lore = file_from_text_to_lore("Example".to_string(), example);
    let html = file_from_lore_to_html(lore);
    println!("{}", html);
}
