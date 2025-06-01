use std::env;

use freedesktop_entry_parser::Entry;
use tokio::fs;

use crate::lookup;

const DEFAULT_XDG_DATA_DIRS: &str = "/usr/local/share/:/usr/share/";

/// Resolve the desktop entry information from the current environment.
///
/// At the moment, only Fuzzel support is implemented.
pub async fn resolve_desktop_entry() -> Result<Option<Entry>, crate::Error> {
    let Some(desktop_file) = env::var_os("FUZZEL_DESKTOP_FILE_ID") else {
        return Ok(None);
    };

    let env_data_dirs = env::var("XDG_DATA_DIRS");
    let data_dirs = env_data_dirs.as_deref().unwrap_or(DEFAULT_XDG_DATA_DIRS);

    let resolved_entry_path =
        lookup::resolve_file(data_dirs, &desktop_file, Some("applications".as_ref()))
            .await
            .ok_or(crate::Error::DesktopEntryNotFound)?;

    let entry_bytes = fs::read(resolved_entry_path)
        .await
        .map_err(|_| crate::Error::DesktopEntryNotFound)?;

    let entry = Entry::parse(entry_bytes)?;

    Ok(Some(entry))
}
