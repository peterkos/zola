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
    let date_str = date.format("%Y-%m-%d").to_string();
    let file_name = date_str.clone() + "-" + title.as_str() + ".md";

    let mut new_post_path = site.content_path;
    new_post_path.push(file_name);

    let content = format!("+++ \n
                   title = \"{}\"
                   date = \"{}\"
                   +++", title, date_str);

    create_file(&new_post_path, content.as_str())?;

    Ok(())
}
