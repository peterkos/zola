use std::path::Path;

use chrono::Local;
use errors::Result;
use site::Site;

use crate::console;
use crate::prompt::read_line;
use utils::fs::*;

pub fn newpost(
    root_dir: &Path,
    config_file: &Path,
) -> Result<()> {
    let site = Site::new(root_dir, config_file)?;

    let title = read_line()?;
    console::info(format!("title: {}", title).as_str());

    // Attempt to create directory if content directory does not exist
    create_directory(&site.content_path)?;

    // Make a new markdown file with specified date format and title
    let date = Local::now();
    let date_str = date.format("%Y-%m-%d").to_string() + "-" + title.as_str();

    let mut new_post_path = site.content_path;
    new_post_path.push(date_str);

    let content = r#"
        Hello, world!
    "#;

    create_file(&new_post_path, content)?;

    Ok(())
}
