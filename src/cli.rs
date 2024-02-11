use std::io;

use crate::app::App;

fn getline<S: AsRef<str>>(prompt: S) -> io::Result<String>{
    println!("{}", prompt.as_ref());
    let mut output = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut output)?;
    Ok(output.lines().next().unwrap().to_string())
}

pub fn run(app: &mut App) -> io::Result<()>{
    if app.no_passphrase() {
        app.set_passphrase(getline("Enter your password")?);
    }

    if app.test_passphrase().is_err() {
        println!("Error: Incorrect password");
        return Ok(());
    }
    app.read();

    app.add_journals();
    // for journal in app.add_journal(journal)
    //     app.new_journal(journal)
    // }

    Ok(())
}
