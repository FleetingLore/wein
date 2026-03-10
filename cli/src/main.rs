mod utils;
mod config;

use std::path::Path;
use std::io;
use crate::config::load_from_config;
use crate::utils::map_lore_files;

fn main() -> io::Result<()> {
    let config = load_from_config();
    let src_dir = Path::new(config.from_lore.as_str());
    let dst_dir = Path::new(config.to_html.as_str());

    map_lore_files(src_dir, dst_dir, &config.base)?;

    println!("done.");
    Ok(())
}
