use std::{ffi::OsString, process::{Command, Stdio}};

use anyhow::{anyhow, Context, Result};
use which::which;

pub fn call(cmd: &str, args: Vec<&OsString>) -> Result<()> {
    let binary = "git";
    info!("Locating `{}` binary for call out.", &binary);

    // get git binary config in case
    which(&binary)
        .with_context(||
            format!("Requires git to be installed, but the command `{}` was not found.", binary)
        )?;
    
    let cmd = if cmd == "gitconfig" {
        "config"
    } else {
        cmd
    };

    debug!("Running git command {:?} with {:?}", &cmd, &args);

    let output = Command::new(binary)
        .arg(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        error!("Command executed with failing error code");
        return Err(anyhow!("Error: {:?}", output));
    }

    Ok(())
}
