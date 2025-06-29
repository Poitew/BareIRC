#![allow(unused)]
use std::collections::HashSet;
use std::net::TcpStream;
use std::io::{
    Write,
    BufRead,
    BufReader,
};

pub enum COMMAND {
    NICK (String),
    USER {username: String, realname: String},
    SERVER (String),
    JOIN (String),
    PART (String),
    PRIVMSG {target: String, message: String},
    NOTICE {target: String, message: String},
    QUIT (String),
    PING (String),
    PONG (String),
    MODE {target: String, mode: String},
    TOPIC {channel: String, topic: String},
    WHO (String),
    WHOIS (String), 
    LIST (String), 
    KICK {channel: String, user: String, reason: String}, 
    INVITE {nick: String, channel: String},
}

pub struct IrcClient {
    nick: String,
    username: String,
    realname: String,
    connection: Option<TcpStream>,
}

impl IrcClient {
    pub fn new() -> Self {
        let nick = String::new();
        let username = String::new();
        let realname = String::new();
        let connection = None;

        IrcClient{nick, username, realname, connection}
    }

    pub fn parse_command(&self, input: &String) -> Result<Vec<String>, String> {
        let commands: HashSet<String> = HashSet::from([
            "NICK", "USER", "SERVER", "JOIN", "PART", "PRIVMSG", "NOTICE", "QUIT", "PING", 
            "PONG", "MODE", "TOPIC", "WHO", "WHOIS", "LIST", "KICK", "INVITE",
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
                    Ok(argv)
                }
                else {
                    return Err("Invalid command, for a list of valid command type /help".to_string());
                }
            }
            else {
                return Err("A command is expected after the prefix '/', for a list of valid command type /help".to_string())
            }
        }
        else {
            return Err("Commands starts with a '/'".to_string())
        }
    }

    pub fn lexer(&self, argv: &[String]) -> Result<COMMAND, ()> {
        use COMMAND::*;

        let Some(cmd) = argv.get(0) else {
            return Err(());
        };
        
        let cmd = cmd.to_uppercase();

        macro_rules! arg {
            ($n: expr) => {
                argv.get($n).ok_or(())?.clone()
            };
        }

        match cmd.as_str() {
            "NICK"      => Ok(NICK(arg!(1))),

            "USER"      => Ok(USER {
                username: arg!(1),
                realname: arg!(2),
            }),

            "SERVER"    => Ok(SERVER(arg!(1))),

            "JOIN"      => Ok(JOIN(arg!(1))),

            "PART"      => Ok(PART(arg!(1))),

            "PRIVMSG"   => Ok(PRIVMSG {
                target: arg!(1),
                message: arg!(2),
            }),

            "NOTICE"    => Ok(NOTICE {
                target: arg!(1),
                message: arg!(2),
            }),

            "QUIT"      => Ok(QUIT(arg!(1))),

            "PING"      => Ok(PING(arg!(1))),

            "PONG"      => Ok(PONG(arg!(1))),

            "MODE"      => Ok(MODE {
                target: arg!(1),
                mode: arg!(2),
            }),

            "TOPIC"     => Ok(TOPIC {
                channel: arg!(1),
                topic: arg!(2),
            }),

            "WHO"       => Ok(WHO(arg!(1))),

            "WHOIS"     => Ok(WHOIS(arg!(1))),

            "LIST"      => Ok(LIST(arg!(1))),

            "KICK"      => Ok(KICK {
                channel: arg!(1),
                user: arg!(2),
                reason: arg!(3),
            }),

            "INVITE"    => Ok(INVITE {
                nick: arg!(1),
                channel: arg!(2),
            }),

            _ => Err(()),
        }
    }

    pub fn execute_command(&mut self, cmd: COMMAND) {
        use COMMAND::*;

        match cmd {
            NICK (nick)                     => self.execute_nick(nick),
            USER {username, realname }      => self.execute_user(username, realname),
            SERVER (server)                 => self.execute_server(server),
            JOIN (channel)                  => self.execute_join(channel),
            PART (channel)                  => self.execute_part(channel),
            PRIVMSG {target, message}       => self.execute_privmsg(target, message),
            NOTICE {target, message}        => self.execute_notice(target, message),
            QUIT (message)                  => self.execute_quit(message),
            PING (server)                   => self.execute_ping(server),
            PONG (server)                   => self.execute_pong(server),
            MODE {target, mode}             => self.execute_mode(target, mode),
            TOPIC {channel, topic}          => self.execute_topic(channel, topic),
            WHO (target)                    => self.execute_who(target),
            WHOIS (targets)                 => self.execute_whois(targets),
            LIST (channels)                 => self.execute_list(channels),
            KICK {channel, user, reason}    => self.execute_kick(channel, user, reason),
            INVITE {nick, channel}          => self.execute_invite(nick, channel),
        }
    }

    pub fn execute_nick(&mut self, nick: String) {
        self.nick = nick;

        println!("You nickname now is: {}", self.nick);
    }

    pub fn execute_user(&mut self, username: String, realname: String) {
        self.username = username;
        self.realname = realname;

        println!("Your username now is: {}", self.username);
        println!("Your realname now is: {}", self.realname);
    }

    pub fn execute_server(&mut self, server: String) {
        if let Ok(connection) = TcpStream::connect(server) {
            self.connection = Some(connection);

            if let Some(stream) = self.connection.as_mut() {
                Self::send_command(stream, format!("NICK {}", self.nick));
                Self::send_command(stream, format!("USER {} 0 * {}", self.username, self.realname));

                let mut reader = BufReader::new(stream);
                let mut line = String::new();

                while let Ok(bytes) = reader.read_line(&mut line) {
                    if bytes == 0 {
                        break;
                    }

                    println!("{}", line);
                    line.clear();
                }
            }
        }
        else {
            eprintln!("Could not connect to the network, check the server address and port");
        }
    }

    pub fn execute_join(&self, channel: String) {

    }

    pub fn execute_part(&self, channel: String) {
        // TODO: implement PART logic
    }

    pub fn execute_privmsg(&self, target: String, message: String) {
        // TODO: implement PRIVMSG logic
    }

    pub fn execute_notice(&self, target: String, message: String) {
        // TODO: implement NOTICE logic
    }

    pub fn execute_quit(&self, message: String) {
        // TODO: implement QUIT logic
    }

    pub fn execute_ping(&self,server: String) {
        // TODO: implement PING logic
    }

    pub fn execute_pong(&self, server: String) {
        // TODO: implement PONG logic
    }

    pub fn execute_mode(&self, target: String, mode: String) {
        // TODO: implement MODE logic
    }

    pub fn execute_topic(&self, channel: String, topic: String) {
        // TODO: implement TOPIC logic
    }

    pub fn execute_who(&self, target: String) {
        // TODO: implement WHO logic
    }

    pub fn execute_whois(&self, targets: String) {
        // TODO: implement WHOIS logic
    }

    pub fn execute_list(&self, channels: String) {
        // TODO: implement LIST logic
    }

    pub fn execute_kick(&self, channel: String, user: String, reason: String) {
        // TODO: implement KICK logic
    }

    pub fn execute_invite(&self, nick: String, channel: String) {
        // TODO: implement INVITE logic
    }

    fn send_command(stream: &mut TcpStream, command: String) -> std::io::Result<()> {
        let full_command = format!("{}\r\n", command);
        stream.write_all(full_command.as_bytes())?;

        println!(">> {}", command);
        Ok(())
    }
}