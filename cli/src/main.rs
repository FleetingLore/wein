mod utils;
mod config;

use std::path::Path;
use std::io;
use crate::config::load_base_from_config;
use crate::utils::map_lore_files;

fn main() -> io::Result<()> {
    let base = load_base_from_config();
    let src_dir = Path::new("static"); 
    let dst_dir = Path::new("docs");

    map_lore_files(src_dir, dst_dir, &base)?;

    println!("done.");
    Ok(())
}
