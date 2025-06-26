use std::io;
use irc::IrcClient;

fn main() {
    let client = IrcClient::new();

    loop {
        let mut command = String::new();

        println!("\nEnter a command: ");
        io::stdin()
            .read_line(&mut command)
            .unwrap();

        match client.parse_command(&command) {
            Some(argv) => match client.lexer(&argv) {
                Ok(tokenized_cmd) => {
                    client.execute_command(tokenized_cmd);
                }

                Err(_) => {
                    eprintln!("Error while lexing the command, check the number of arguments");
                }
            },

            None => {
                eprintln!("Error while parsing the command");
            }
        }
    }
}
