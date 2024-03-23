mod tui;
use std::{path::PathBuf, io};
use clap::Parser;
use home::home_dir;
mod app;
mod cli;
use app::App;

fn print_files(path: PathBuf) {
    println!("{}", path.as_os_str().to_str().unwrap());
}

pub fn append_home_dir(vec: [&str; 3]) -> PathBuf {
    let mut path = PathBuf::from(format!("{}", home_dir().unwrap().to_str().unwrap()));
    for item in vec {
        path = path.join(item);
    }

    path
}

pub fn default_directory() -> PathBuf {
    append_home_dir([".local", "share", "jou"])
}

/// A good journal application a day, make therapy go away
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to journal directory. Unique for each password
    #[arg(short, long,default_value=default_directory().into_os_string())]
    path: PathBuf,

    /// Pass the passphrase non-interactively
    #[arg(short='P', long)]
    passphrase: Option<String>,

    /// Add a journal entry
    #[arg(short='a', long)]
    add: Vec<String>,

    /// List in non-interactive mode
    #[arg(short='l', long)]
    list: bool,
}

impl Args {
    pub fn is_cli(&self) -> bool {
        self.list || !self.add.is_empty()
    }
}

fn main() -> io::Result<()>{
    let args = Args::parse();
    let is_cli = args.is_cli();
    let mut app = App::new(args)?;
    if is_cli {
        cli::run(&mut app)?;
    } else {
        tui::run(&mut app)?;
    }
    Ok(())
}
