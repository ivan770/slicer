use std::{
    borrow::Cow,
    env,
    ffi::OsStr,
    future,
    path::{Path, PathBuf},
    pin::pin,
};

use camino::Utf8PathBuf;
use futures_util::{StreamExt, TryStreamExt, stream};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

/// Resolve an absolute path from the provided executable name.
///
/// Utilizes the `PATH` environment variable to perform the lookup.
pub async fn resolve_executable(executable: &str) -> Result<Utf8PathBuf, crate::Error> {
    // Scanning the current directory is explicitly not implemented,
    // as this program assumes that you run it from something like a global menu.
    let path = env::var("PATH").map_err(|_| crate::Error::AppNotFound)?;

    resolve_file(&path, executable.as_ref(), None)
        .await
        .ok_or(crate::Error::AppNotFound)
}

/// An imperfect implementation of colon-delimited path lookups.
///
/// Works great on NixOS, on other distros mileage may vary.
pub async fn resolve_file(
    haystack: &str,
    needle: &OsStr,
    suffix: Option<&OsStr>,
) -> Option<Utf8PathBuf> {
    let paths = haystack.split(':').map(|path| {
        if let Some(suffix) = suffix {
            let mut path = PathBuf::from(path);
            path.push(suffix);
            Cow::Owned(path)
        } else {
            Cow::Borrowed(Path::new(path))
        }
    });

    let stream = stream::iter(paths)
        .then(fs::read_dir)
        .map_ok(ReadDirStream::new)
        .try_flatten()
        .filter_map(async |v| v.ok())
        .filter(|entry| future::ready(entry.file_name() == needle));

    let mut stream = pin!(stream);

    let entry = stream.next().await?;

    Utf8PathBuf::from_path_buf(entry.path()).ok()
}
