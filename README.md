# Slicer

Run desktop applications as systemd services.

Inspired by [uwsm](https://github.com/Vladimir-csp/uwsm) and [runapp](https://github.com/c4rlo/runapp) projects.

## Usage

While it's possible to use Slicer as a standalone application, you would usually integrate it
as part of your window manager launch menu.

For example, to use Slicer with Fuzzel, modify your launch command with the `--launch-prefix` flag:

```sh
fuzzel --launch-prefix="slicer -- "
```

### Slices

By default, Slicer utilizes uwsm's `app-graphical.slice` slice to launch your applications.
You can customize this behavior with the `-s` flag:

```sh
fuzzel --launch-prefix="slicer -s session-graphical -- "
```

Supported slices:

* `app`
* `session`
* `background`
* `app-graphical`
* `session-graphical`
* `background-graphical`

See [systemd documentation](https://systemd.io/DESKTOP_ENVIRONMENTS/) for more information
on how to choose a correct slice for your application.

## Build from source

This repository contains a Nix flake with a package within it,
which you can utilize for an easy integration into your NixOS config:

```sh
nix build github:ivan770/slicer
```

Otherwise, if you don't have Nix installed, use `cargo build --release` to build the binary.
