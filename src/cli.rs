use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    ///Start task
    #[structopt()]
    Add { task: String},
    #[structopt()]
    Done { position: usize},
    List,
}
#[derive(Debug, StructOpt)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}