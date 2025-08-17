use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Init,
   Create{ branch: String },
   Commit{ message: String },
   Cat{ hash: String },
   Delete{ branch: String },
   Log,
   Revert{ branch_location: String, branch_level: String }
}

fn main() {
     let cli = Cli::parse();
}
