# filearchy

A file manager for Wayland — forked from [cosmic-files](https://github.com/pop-os/cosmic-files) with custom features for omarchy/Hyprland workflows.

## Features (fork additions)

- **Custom MIME icons** — per-type icons for source files, documents, media, and more
- **Extended archive support** — compress/extract `.tar.bz2`, `.tar.lz4`, `.tar.xz`, `.tar.zst` in addition to `.tgz` and `.zip`
- **Terminal app support** — files with `Terminal=true` in their `.desktop` entry (e.g. neovim, vim, nano) open inside the configured terminal emulator
- **omarchy clipboard keybinds** — Super+C/V/X and Ctrl/Shift+Insert work in the file list and location bar
- **Location bar focus tracking** — real `on_focus`/`on_unfocus` state; typing in the bar extends the path without erasing it
- **Search stability** — minimum 2-char search term, deduplication, no rescans on filesystem events while searching
- **Search context menu** — right-click actions available on search results
- **List view default** — opens in list view; `pkill -9 filearchy` works

## Build

```sh
git clone https://github.com/pop-os/cosmic-files filearchy
cd filearchy
cargo build --release
# Binary: target/release/filearchy
```

### Dev workflow

```sh
just run        # build release + run with debug logs (foreground)
just dev        # cargo fmt + just run
```

## Install (symlink-based)

```sh
# Wrapper script at ~/.local/bin/filearchy pointing to target/release/filearchy
# Hyprland keybind: bindd = SUPER, F, File manager, exec, uwsm-app -- ~/.local/bin/filearchy
```

## Process management

```sh
pkill -9 filearchy          # kill all instances
systemctl --user list-units --state=active | grep filearchy   # list scope units
```

Because the app uses `exit_on_close`, processes persist after windows are closed. Press Super+F repeatedly and multiple instances accumulate — kill them periodically or restart the session.

## Upstream

This is a fork of [pop-os/cosmic-files](https://github.com/pop-os/cosmic-files), the file manager for the [COSMIC desktop](https://github.com/pop-os/cosmic-epoch). Upstream changes are periodically rebased in.

## License

GPL-3.0-only — see [LICENSE](LICENSE)
