mod cli;
mod tasks;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::*;
use std::path::PathBuf;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let args = CommandLineArgs::from_args();
    
    let journal_file = args.journal_file
        .or_else(find_default_journal_file)
        .expect("Filed to find journal file");

    match args.action {
        Add { task } => add_task(journal_file, Task::new(task)),
        Done { position } => complete_task(journal_file, position),
        List => list_tasks(journal_file),
    }
    .expect("Failed to complete action");

    Ok(())
}
