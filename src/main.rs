use std::io;
use irc::IrcClient;

fn main() {
    let mut client = IrcClient::new();

    println!("Usage:
        \n 1. /NICK <nickname>
        \n 2. /USER <username> <realname>
        \n 3. /SERVER <hostname>:<port>"
    );

    loop {
        let mut command = String::new();

        println!("\nEnter command: ");
        io::stdin()
            .read_line(&mut command)
            .unwrap();

        match client.parse_command(&command) {
            Ok(argv) => match client.lexer(&argv) {
                Ok(tokenized_cmd) => {
                    client.execute_command(tokenized_cmd);
                }

                Err(_) => {
                    eprintln!("Error while lexing the command, check the number of arguments");
                }
            },

            Err(e) => {
                eprintln!("Error while parsing the command: \n{e}");
            }
        }
    }
}
