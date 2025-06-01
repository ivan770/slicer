use std::{
    borrow::Cow,
    env,
    fmt::{self, Write},
};

use argh::FromArgValue;
use ascii::AsciiStr;
use camino::Utf8Path;
use freedesktop_entry_parser::Entry;
use systemd_zbus::{ManagerProxy, Mode};
use zbus::zvariant::Value;

#[derive(Copy, Clone, Default)]
pub enum Slice {
    #[default]
    AppGraphical,
    SessionGraphical,
    BackgroundGraphical,

    App,
    Session,
    Background,
}

impl AsRef<str> for Slice {
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Slice::AppGraphical => "app-graphical.slice",
            Slice::SessionGraphical => "session-graphical.slice",
            Slice::BackgroundGraphical => "background-graphical.slice",
            Slice::App => "app.slice",
            Slice::Session => "session.slice",
            Slice::Background => "background.slice",
        }
    }
}

impl FromArgValue for Slice {
    #[inline]
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "app-graphical" => Ok(Self::AppGraphical),
            "session-graphical" => Ok(Self::SessionGraphical),
            "background-graphical" => Ok(Self::BackgroundGraphical),
            "app" => Ok(Self::App),
            "session" => Ok(Self::Session),
            "background" => Ok(Self::Background),
            _ => Err(format!("unknown slice value: {value}")),
        }
    }
}

/// Create and start a new systemd transient unit.
pub async fn start_transient_unit(
    unit_name: &str,
    executable_path: &str,
    args: &[String],
    slice: Slice,
    desktop_entry: Option<&Entry>,
) -> Result<(), crate::Error> {
    let dbus_connection = zbus::Connection::session().await?;
    let proxy = ManagerProxy::new(&dbus_connection).await?;

    let exec_start = [(executable_path, args, false)];

    let mut properties = Vec::with_capacity(6);

    properties.extend([
        ("Type", Value::Str("exec".into())),
        ("ExecStart", Value::Array(exec_start[..].into())),
        // Corresponds to the "--collect" flag.
        ("CollectMode", Value::Str("inactive-or-failed".into())),
        ("ExitType", Value::Str("cgroup".into())),
        ("Slice", Value::Str(slice.as_ref().into())),
    ]);

    if let Some(desktop_entry) = desktop_entry {
        let desktop_entry_section = desktop_entry.section("Desktop Entry");

        let name = desktop_entry_section.attr("Name");
        let comment = desktop_entry_section.attr("Comment");

        let description = match (name, comment) {
            (Some(name), Some(comment)) => Some(Cow::Owned(format!("{name} - {comment}"))),
            (Some(name), None) => Some(Cow::Borrowed(name)),
            (None, Some(comment)) => Some(Cow::Borrowed(comment)),
            (None, None) => None,
        };

        if let Some(description) = description {
            properties.push(("Description", Value::Str(description.into())));
        }
    }

    proxy
        .start_transient_unit(unit_name, Mode::Fail, &properties, &[])
        .await?;

    Ok(())
}

/// Generate a new systemd unit name for the provided executable from the current environment.
pub fn create_unit_name(executable_path: &Utf8Path) -> Result<String, fmt::Error> {
    let mut unit_name = String::with_capacity(48);
    write!(unit_name, "app")?;

    let current_desktop = env::var("XDG_CURRENT_DESKTOP");

    if let Ok(current_desktop) = current_desktop {
        write!(unit_name, "-{}", current_desktop)?;
    }

    if let Some(bin) = executable_path.file_name() {
        if let Ok(ascii) = AsciiStr::from_ascii(bin.as_bytes()) {
            write!(unit_name, "-{}", ascii)?;
        }
    }

    let mut rng = fastrand::Rng::new();
    let mut num_buf = itoa::Buffer::new();

    let app_id = rng.u32(..);
    let formatted_app_id = num_buf.format(app_id);
    write!(unit_name, "-{}.service", formatted_app_id)?;

    Ok(unit_name)
}
