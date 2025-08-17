use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author,version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Init,
   Create,
   Commit,
   Cat,
   Delete,
   Log,
   Revert
}
fn main() {
     let cli = Cli::parse();
}
