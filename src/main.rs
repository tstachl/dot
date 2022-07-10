#![allow(unused)]

#[macro_use]
extern crate log;

use anyhow::{Context, Result};
use std::ffi::OsString;
use std::path::PathBuf;

mod command;
mod git;

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");

    let matches = command::command().get_matches();
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            debug!("executing init with {:?}", &sub_matches);
        }
        Some((cmd, sub_matches)) => {
            debug!("executing {:?} with {:?}", &cmd, &sub_matches);

            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            
            debug!("Running git command {:?} with {:?}", &cmd, &args);
            git::call(cmd, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}

