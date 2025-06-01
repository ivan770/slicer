mod desktop;
mod lookup;
mod systemd;

use std::fmt;

use argh::FromArgs;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("The provided command is missing the application to launch")]
    Arg0Missing,

    #[error("The executable was not found in your current $PATH")]
    AppNotFound,

    #[error("Desktop entry ID was provided, but the entry was not found")]
    DesktopEntryNotFound,

    #[error("Desktop entry parsing failed: {0}")]
    MalformedDesktopEntry(#[from] freedesktop_entry_parser::ParseError),

    #[error("D-Bus: {0}")]
    Dbus(#[from] zbus::Error),

    #[error("Unable to generate the unit name: {0}")]
    Fmt(#[from] fmt::Error),
}

/// Run your apps as systemd units.
#[derive(FromArgs)]
struct Command {
    /// systemd slice to utilize
    #[argh(short = 's', option)]
    slice: Option<systemd::Slice>,

    /// application to launch
    #[argh(positional, greedy)]
    args: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let command: Command = argh::from_env();

    let executable = command.args.first().ok_or(Error::Arg0Missing)?;
    let slice = command.slice.unwrap_or_default();

    let (executable_path, resolved_desktop_entry) = tokio::try_join!(
        lookup::resolve_executable(executable.as_ref()),
        desktop::resolve_desktop_entry(),
    )?;

    let unit_name = systemd::create_unit_name(&executable_path).map_err(Error::Fmt)?;

    systemd::start_transient_unit(
        &unit_name,
        executable_path.as_str(),
        &command.args,
        slice,
        resolved_desktop_entry.as_ref(),
    )
    .await?;

    Ok(())
}
