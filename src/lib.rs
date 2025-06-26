use std::collections::HashSet;
use std::net::TcpStream;

pub enum Command {
    NICK (String),
    USER {username: String, realname: String},
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

pub struct IrcClient;

impl IrcClient {
    pub fn new() -> Self {
        IrcClient
    }

    pub fn parse_command(&self, input: &String) -> Option<Vec<String>> {
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

    pub fn lexer(&self, argv: &[String]) -> Result<Command, ()> {
        use Command::*;

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

    pub fn execute_command(&self, cmd: Command) {
        use Command::*;

        match cmd {
            NICK (nick)                     => Self::execute_nick(nick),
            USER {username, realname }      => Self::execute_user(username, realname),
            JOIN (channel)                  => Self::execute_join(channel),
            PART (channel)                  => Self::execute_part(channel),
            PRIVMSG {target, message}       => Self::execute_privmsg(target, message),
            NOTICE {target, message}        => Self::execute_notice(target, message),
            QUIT (message)                  => Self::execute_quit(message),
            PING (server)                   => Self::execute_ping(server),
            PONG (server)                   => Self::execute_pong(server),
            MODE {target, mode}             => Self::execute_mode(target, mode),
            TOPIC {channel, topic}          => Self::execute_topic(channel, topic),
            WHO (target)                    => Self::execute_who(target),
            WHOIS (targets)                 => Self::execute_whois(targets),
            LIST (channels)                 => Self::execute_list(channels),
            KICK {channel, user, reason}    => Self::execute_kick(channel, user, reason),
            INVITE {nick, channel}          => Self::execute_invite(nick, channel),
        }
    }

    pub fn execute_nick(nick: String) {
        // TODO: implement NICK logic
    }

    pub fn execute_user(username: String, realname: String) {
        // TODO: implement USER logic
    }

    pub fn execute_join(channel: String) {
        let connection = TcpStream::connect("irc.libera.chat:6667");

        println!("Connesso!");
    }

    pub fn execute_part(channel: String) {
        // TODO: implement PART logic
    }

    pub fn execute_privmsg(target: String, message: String) {
        // TODO: implement PRIVMSG logic
    }

    pub fn execute_notice(target: String, message: String) {
        // TODO: implement NOTICE logic
    }

    pub fn execute_quit(message: String) {
        // TODO: implement QUIT logic
    }

    pub fn execute_ping(server: String) {
        // TODO: implement PING logic
    }

    pub fn execute_pong(server: String) {
        // TODO: implement PONG logic
    }

    pub fn execute_mode(target: String, mode: String) {
        // TODO: implement MODE logic
    }

    pub fn execute_topic(channel: String, topic: String) {
        // TODO: implement TOPIC logic
    }

    pub fn execute_who(target: String) {
        // TODO: implement WHO logic
    }

    pub fn execute_whois(targets: String) {
        // TODO: implement WHOIS logic
    }

    pub fn execute_list(channels: String) {
        // TODO: implement LIST logic
    }

    pub fn execute_kick(channel: String, user: String, reason: String) {
        // TODO: implement KICK logic
    }

    pub fn execute_invite(nick: String, channel: String) {
        // TODO: implement INVITE logic
    }
}