use std::{fs, io};
use std::path::{Path, PathBuf};
use html::LoreHtml;
use parser::file::{file_from_lore_to_html, file_from_text_to_lore};

/// 处理单个文件：从 .lore 文件生成 .html 文件
fn process_lore_file(src: &Path, dst: &Path, base: &str) -> io::Result<()> {
    println!("处理文件: {:?}", src);

    // 1. 读取 .lore 文件
    let content = fs::read_to_string(src)?;

    // 2. 解析 lore 内容
    let lore_data = file_from_text_to_lore(content);

    // 3. 转换为 HTML
    let html_content = file_from_lore_to_html(lore_data, base);

    // 4. 生成完整 HTML
    let file_name = src
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let final_html: String = LoreHtml::new(
        file_name,
        base.to_string(),
        html_content
    ).into();

    // 6. 修改输出文件名为 .html
    let dst_html = dst.with_extension("html");

    // 7. 写入文件
    fs::write(dst_html, final_html)?;

    Ok(())
}

/// 确保目录存在
fn ensure_dir(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 获取相对路径
fn get_relative_path(full_path: &Path, base: &Path) -> PathBuf {
    full_path.strip_prefix(base)
        .unwrap_or(full_path)
        .to_path_buf()
}

/// 主函数：遍历目录并处理所有 .lore 文件
pub fn map_lore_files(src_dir: &Path, dst_dir: &Path, base: &str) -> io::Result<()> {
    ensure_dir(dst_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();

        if src_path
            .is_dir() {
            // 递归处理子目录
            let rel_path = get_relative_path(&src_path, src_dir);
            let dst_path = dst_dir
                .join(rel_path);
            map_lore_files(&src_path, &dst_path, base)?;
        } else {
            // 只处理 .lore 文件
            if src_path
                .extension()
                .and_then(|ext| ext.to_str()) == Some("lore")
            {
                let rel_path = get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                ensure_dir(dst_path.parent().unwrap())?;
                process_lore_file(&src_path, &dst_path, base)?;
            }
        }
    }

    Ok(())
}