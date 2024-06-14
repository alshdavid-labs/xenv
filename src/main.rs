mod dotenv;
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
  let Ok(env) = dotenv::parse_file(&cmd.env_file_path) else {
    println!("Failed to read env file");
    std::process::exit(1);
  };

  let mut output = String::new();

  for (key, value) in env {
    output += &format!("export {}={};", key, value);
  }

  print!("{}", output);
}
