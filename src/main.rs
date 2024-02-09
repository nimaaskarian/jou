mod tui;
use std::{fs, path::PathBuf, io::{self, Write}};
use clap::Parser;
use home::home_dir;
mod app;
use app::App;

fn print_files(path: PathBuf) {
    println!("{}", path.as_os_str().to_str().unwrap());
}

pub fn append_home_dir(str:&str) -> PathBuf {
    PathBuf::from(format!("{}/{}", home_dir().unwrap().to_str().unwrap(), str))
}

pub fn default_directory() -> PathBuf {
    append_home_dir( ".local/share/jou")
}


/// A good journal application a day, make therapy go away
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long,default_value=default_directory().into_os_string())]
    path: PathBuf,
}

fn getline<S: AsRef<str>>(prompt: S) -> io::Result<String>{
    println!("{}", prompt.as_ref());
    let mut output = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut output)?;
    Ok(output.lines().next().unwrap().to_string())
}

fn main() -> io::Result<()>{
    let args = Args::parse();
    let passphrase = getline("Gimme passphrase")?;
    let mut app = App::new(args, passphrase);

    if app.test_passphrase().is_err() {
        println!("Incorrect password")
    }
    // let dir = Directory::new(args.path).unwrap();
    // let path = dir.new_path()?;
    // let encryption = Encryption::new(passphrase);

    // dir.read(|path| {
    //     let string = fs::read(path).unwrap();
    //     println!("{}", encryption.decrypt(string).unwrap());
    // })?;

    // let text = getline("Gimme content")?;
    // if !text.is_empty() {
    //     let encrypted = encryption.encrypt(text).unwrap();
    //     fs::write(path, encrypted);
    // }
    Ok(())
}
