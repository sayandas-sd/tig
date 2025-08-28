use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Init,
   Create{ branch: String },
   Commit{ 
        #[arg(short, long, default_value = "")]
        message: String 
    },
   Cat{ hash: String },
   Delete{ branch: String },
   Log,
   Revert{ 
        #[arg(short, long, default_value = "1")]
        branch_location: u32, 
        branch_level: String 
   }
}

fn main() {
     let cli = Cli::parse();

    
    match &cli.command {
            Commands::Init => println!("init repo"),
            Commands::Create { branch } => println!("create branch {}", branch),
            Commands::Commit { message } => println!("commit message {}", message),
            Commands::Cat { hash } => println!("objects {}", hash),
            Commands::Delete { branch } => println!("delete branch {}", branch),
            Commands::Log => println!("Show log"),
            Commands::Revert { branch_location, branch_level } => {
                println!("Revert branch {} at level {}", branch_level, branch_location);
            }
        }
}
