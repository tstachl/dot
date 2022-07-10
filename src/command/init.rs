use clap::{arg, Command};

pub fn command() -> Command<'static> {
    Command::new("init")
        .about("Initialize a new, empty repository for tracking dotfiles.")
        .long_about("Initialize a new, empty repository for tracking dotfiles. The repository is stored in $HOME/.dot/repo.git. By default, $HOME will be used as the work-tree, but this can be overridden with the -w option. dot can be forced to overwrite an existing repository by providing the -f option.")
        .arg(
            arg!(force: -f <FORCE> "Use '-f' if you want to force it to be overwritten.")
                .required(false)
        )
}
