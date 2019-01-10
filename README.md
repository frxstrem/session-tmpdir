# session-tmp

`session-tmpdir` is a simple program for Linux, written in Rust, that allows you to create temporary, process-specific filesystems.

> **Note!** This program is very much just a proof of concept, it doesn't do much checking in terms of security and I am in no way liable for any damages that may be caused by use of this program. Use at your own risk.

## Requirements

* Linux
* Rust and `cargo` already installed

## How to install

1. Clone this repository
2. Run `cargo install --path .`
3. Run `sudo setcap cap_sys_admin+ep ~/.cargo/bin/session-tmpdir` to set `CAP_SYS_ADMIN` capability for the program.
4. Move `~/.cargo/bin/session-tmpdir` to somewhere in your path.

## How to run

Run as `session-tmpdir -d <dir> [cmd] [args...]`. This will create a separate mount namespace (i.e., "filesystem tree"), mount `tmpfs` in the given directory and run the given command with the given arguments. If `cmd` is not given, it will try to run `/bin/bash` by default.