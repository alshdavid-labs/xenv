#![allow(legacy_derive_helpers)]

mod dotenv;
use clap::Parser;
use clap_complete::Shell;
use std::path::PathBuf;

/// --------
/// - xenv -
/// --------
/// Expand .env files so they can be used in shell environments
/// Shell Usage:
///   Bash:         eval $(xenv ./.env)
///   PowerShell:   Invoke-Expression &{ xenv .\.env }
#[clap(verbatim_doc_comment)]
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

  let mut shell = None::<String>;
  if let Some(s) = cmd.shell {
    shell = Some(s)
  } else if let Some(s) = Shell::from_env() {
    match s {
      Shell::Bash => { shell = Some("bash".into())},
      Shell::Elvish => { shell = Some("elvish".into())},
      Shell::Fish => { shell = Some("fish".into())},
      Shell::PowerShell => { shell = Some("powershell".into())},
      Shell::Zsh => { shell = Some("zsh".into())},
      _ => {},
    };
  } else {
    println!("Failed to detect shell");
    std::process::exit(1);
  }

  let Some(shell) = shell else {
    println!("Failed to detect shell");
    std::process::exit(1);
  };

  if &shell == "bash" || &shell == "zsh" {
    for (key, value) in env {
      output += &format!("export {}={};", key, value);
    }
  }

  else if &shell == "powershell" {
    for (key, value) in env {
      output += &format!("Set-Item \"env:{}\" \"{}\";", key, value);
    }
  }

  print!("{}", output);
}
