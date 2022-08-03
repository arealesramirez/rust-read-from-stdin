use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    println!("Reading a single line... \n");
    read_single_line()?;

    println!("Reading multiple lines... \n");
    read_muliple_lines()?;

    // the lock is released after it goes out of scope
    Ok(())
}

fn read_single_line() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input)?;

    println!("input {} ", user_input);
 
    Ok(())
}

fn read_muliple_lines() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line?;

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
    }

    println!("\nMulti-line user input \n{}", user_input);

    Ok(())
}
