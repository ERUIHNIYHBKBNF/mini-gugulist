use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add to gugu list.
    Add {
        #[structopt()]
        task: String,
    },
    /// Remove from gugu list.
    Done {
        #[structopt()]
        position: usize,
    },
    /// Show gugu list.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mini-gugulist",
    about = "A simple todo list CLI"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different list file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
