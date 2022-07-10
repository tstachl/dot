use clap::{arg, ArgAction, Command};

mod init;

pub fn command() -> Command<'static> {
    Command::new("dot")
        .author("Thomas Stachl <thomas@stachl.me>")
        .version("0.0.1")
        .about("A cross-platform, single dependency, dotfile manager.")
        .arg(
            arg!(--"dot-dir" <DIR> "Override the dot directory. dot stores its configurations relative to this directory.")
                .default_value("$HOME/.config/dot").required(false)
        )
        .arg(
            arg!(--"dot-data" <DATA> "Override the dot data directory. dot stores its data relative to this directory.")
                .default_value("$HOME/.local/share/dot").required(false)
        )
        .arg(
            arg!(--"dot-repo" <REPO> "Override the location of the dot repository.")
                .default_value("$HOME/.local/share/dot/repo.git").required(false)
        )
        .arg(
            arg!(--"dot-config" <CONFIG> "Override the location of the dot configuration file.")
                .default_value("$HOME/.config/dot/config").required(false)
        )
        .arg(
            arg!(--"dot-encrypt" <ENCRYPT> "Override the location of the dot encryption configuration.")
                .default_value("$HOME/.config/dot/encrypt").required(false)
        )
        .arg(
            arg!(--"dot-archive" <ARCHIVE> "Override the location of the dot encrypted files archive.")
                .default_value("$HOME/.local/share/dot/archive").required(false)
        )
        .arg(
            arg!(--"dot-bootstrap" <BOOTSTRAP> "Override the location of the dot bootstrap program.")
                .default_value("$HOME/.config/dot/bootstrap").required(false)
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            init::command()
        )
}
