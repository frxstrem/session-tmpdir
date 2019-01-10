use std::error::Error;
use std::ffi::CString;
use std::fs;

use clap::clap_app;

use caps::{CapSet, Capability};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{execv, geteuid};

const NO_STR: Option<&'static str> = None;

fn main() -> Result<(), Box<dyn Error>> {
    // specify program arguments
    let app = clap_app!(("session-tmpdir") =>
        (@arg dir: -d --dir +takes_value *)
        (@arg cmd: ...)
    );

    // parse arguments
    let matches = app.get_matches();

    let dir = fs::canonicalize(matches.value_of("dir").unwrap())?;
    let cmd = match matches.values_of("cmd") {
        Some(args) => args.collect(),
        None => Vec::new(),
    };

    // check if CAP_SYS_ADMIN is effective (we need it to call unshare() and mount())
    if !caps::has_cap(None, CapSet::Permitted, Capability::CAP_SYS_ADMIN)? {
        return Err("Missing capability: CAP_SYS_ADMIN".into());
    }

    // ensure that CAP_SYS_ADMIN is not inheritable unless we are root
    if geteuid().is_root() {
        caps::drop(None, CapSet::Inheritable, Capability::CAP_SYS_ADMIN)?;
    }

    // create new mount namespace for process
    unshare(CloneFlags::CLONE_NEWNS)?;

    // remount root mountpoint as rprivate in current mount namespace
    mount(
        NO_STR,
        "/",
        NO_STR,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        NO_STR,
    )?;

    // create new tmpfs mountpoint
    mount(
        Some("tmpfs"),
        &dir,
        Some("tmpfs"),
        MsFlags::MS_MGC_VAL,
        NO_STR,
    )?;

    // make tmpfs private
    mount(
        NO_STR,
        &dir,
        NO_STR,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        NO_STR,
    )?;

    // execute command
    if cmd.is_empty() {
        // TODO: find and execute default shell
        let path = CString::new("/bin/bash")?;
        let argv = &[path.clone()];
        execv(&path, argv)?;
    } else {
        let path = CString::new(*cmd.get(0).unwrap())?;
        let argv = cmd
            .iter()
            .map(|arg| CString::new(*arg))
            .collect::<Result<Vec<_>, _>>()?;
        execv(&path, &argv)?;
    }

    Ok(())
}
