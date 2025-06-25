use std::io;
use std::net;
use std::collections::HashSet;

fn main() {
    loop {
        let mut command = String::new();

        println!("\nEnter a command: ");
        io::stdin()
            .read_line(&mut command)
            .unwrap();

        if let Some(argv) = parse_command(&command) {
            execute_command(argv);
        }
    }
}

fn parse_command(input: &String) -> Option<Vec<String>> {
    let commands: HashSet<String> = HashSet::from([
        "NICK", "USER", "JOIN", "PART", "PRIVMSG", "NOTICE", "QUIT", "PING", 
        "PONG", "MODE", "TOPIC", "WHO", "WHOIS", "LIST", "KICK", "INVITE"
    ])
    .iter()
    .map(|s| s.to_string())
    .collect();

    if let Some(cmd) = input.strip_prefix('/') {
        let argv: Vec<String> = cmd
            .split_whitespace()
            .map(|s| s.to_uppercase())
            .collect();
        
        if let Some(command) = argv.first() {
            if commands.contains(command) {
                Some(argv)
            }
            else {
                eprintln!("Error: Invalid command, for a list of valid command type /help");
                None
            }
        }
        else {
            eprintln!("Error: A command is expected after the prefix '/', for a list of valid command type /help");
            None
        }
    }
    else {
        eprintln!("Error: Commands starts with a '/' ");
        None
    }
}

fn execute_command(argv: Vec<String>) {
    match argv[0] {
        "NICK" =>
        "USER" =>
        "JOIN" =>
        "PART" =>
        "PRIVMSG" =>
        "NOTICE" =>
        "QUIT" =>
        "PING" => 
        "PONG" =>
        "MODE" =>
        "TOPIC" =>
        "WHO" =>
        "WHOIS" =>
        "LIST" =>
        "KICK" =>
        "INVITE"=>
        _ => unreachable!("Unknow command") // Case is unreachable because there already is a check in 'parse_command'
    }
}