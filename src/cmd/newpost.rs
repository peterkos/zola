use std::path::Path;

use errors::{Error, Result};
use site::Site;

use crate::console;
use crate::prompt::read_line;

pub fn newpost(
    root_dir: &Path,
    config_file: &Path,
) -> Result<()> {
    let mut site = Site::new(root_dir, config_file)?;

    let input = read_line()?;

    console::info(format!("input: {}", input).as_str());



    Ok(())

}
