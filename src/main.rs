use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Commands {
  /// The env file to expand
  env_file_path: PathBuf,

  /// The shell environment to target (automatically detected)
  #[arg(short = 's', long = "shell")]
  shell: Option<String>,
}

fn main() {
  let cmd = Commands::parse();
  dbg!(&cmd);
}
