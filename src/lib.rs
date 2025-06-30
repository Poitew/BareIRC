#![allow(unused)]
use std::collections::HashSet;
use std::net::TcpStream;
use std::thread;
use std::io::{
    Write,
    BufRead,
    BufReader,
};

pub enum COMMAND {
    EXIT,
    HELP,
    NICK (String),
    USER {username: String, realname: String},
    SERVER (String),
    JOIN (String),
    PART (String),
    PRIVMSG {target: String, message: String},
    NOTICE {target: String, message: String},
    QUIT (String),
    MODE {target: String, mode: String},
    TOPIC {channel: String, topic: String},
    WHO (String),
    WHOIS (String), 
    LIST, 
    KICK {channel: String, user: String, reason: String}, 
    INVITE {nick: String, channel: String},
}

pub struct IrcClient {
    pub active: bool,
    nick: String,
    username: String,
    realname: String,
    connection: Option<TcpStream>,
}

impl IrcClient {
    pub fn new() -> Self {
        let active = true;
        let nick = String::new();
        let username = String::new();
        let realname = String::new();
        let connection = None;

        IrcClient{active, nick, username, realname, connection}
    }

    pub fn parse_command(&self, input: &String) -> Result<Vec<String>, String> {
        let commands: HashSet<String> = HashSet::from([
            "EXIT", "HELP", "NICK", "USER", "SERVER", "JOIN", "PART", "PRIVMSG", "NOTICE", "QUIT", 
            "MODE", "TOPIC", "WHO", "WHOIS", "LIST", "KICK", "INVITE",
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
            "EXIT"      => Ok(EXIT),

            "HELP"      => Ok(HELP),

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

            "LIST"      => Ok(LIST),

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
            EXIT                            => self.execute_exit(),
            HELP                            => self.execute_help(),
            NICK (nick)                     => self.execute_nick(nick),
            USER {username, realname }      => self.execute_user(username, realname),
            SERVER (server)                 => self.execute_server(server),
            JOIN (channel)                  => self.execute_join(channel),
            PART (channel)                  => self.execute_part(channel),
            PRIVMSG {target, message}       => self.execute_privmsg(target, message),
            NOTICE {target, message}        => self.execute_notice(target, message),
            QUIT (message)                  => self.execute_quit(message),
            MODE {target, mode}             => self.execute_mode(target, mode),
            TOPIC {channel, topic}          => self.execute_topic(channel, topic),
            WHO (target)                    => self.execute_who(target),
            WHOIS (targets)                 => self.execute_whois(targets),
            LIST                            => self.execute_list(),
            KICK {channel, user, reason}    => self.execute_kick(channel, user, reason),
            INVITE {nick, channel}          => self.execute_invite(nick, channel),
        }
    }



    // Helper functions
    fn send_command(stream: &mut TcpStream, command: String) -> std::io::Result<()> {
        let full_command = format!("{command}\r\n");
        stream.write_all(full_command.as_bytes())?;

        println!(">> {}", command);
        Ok(())
    }

    fn listen_messages(stream: &mut TcpStream) {
        let stream_clone = stream.try_clone().unwrap();

        let mut reader = BufReader::new(stream_clone);
        let mut line = String::new();

        thread::spawn(move || {
            while let Ok(bytes) = reader.read_line(&mut line) {
                if bytes == 0 {
                    break;
                }

                println!("{}", line);
                line.clear();
            }
        });
    }
    
    fn with_stream<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TcpStream),
    {
        if let Some(stream) = self.connection.as_mut() {
            f(stream);
        } 
        else {
            eprintln!("You are not connected to any network!");
        }
    }



    // Commands executor
    pub fn execute_exit(&mut self) {
        self.active = false;
    }

    pub fn execute_help(&mut self) {
        println!("\t/nick <nickname>: set personal nickname.");
        println!("\t/user <username> <realname>: set personal username and realname.");
        println!("\t/server <hostname>:<port>: join a network, requires to have nickname, username, and realname already set.");
        println!("\t/join #<server>: join a server, all server must start with an hash sign.");
        println!("\t/privmsg #<target> <message>: send a message in the specified server, or to the specified user.");
        println!("\t/notice #<target> <message>: send a message in the specified server, or to the specified user.");
        println!("\t/part #<server>: leave the specified <server>.");
        println!("\t/quit #<reason>: leave the network, <reason> is mandatory.");
        println!("\t/topic #<channel> #<topic>: changes the topic for the specified server (channel).");
        println!("\t/who <target>: return a list of users who match <target>.");
        println!("\t/whois <users>: print infos about the comma-separated list of users.");
        println!("\t/list: list all the servers in the current network.");
        println!("\t/kick <channel> <target> :<reason>: kick <target> from <channel> with the given <reason>.");
        println!("\t/invite <nickname> <channel>: invite <nickname> to <channel>.");
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

                Self::listen_messages(stream);
            }
        }
        else {
            eprintln!("Could not connect to the network, check the server address and port");
        }
    }


    pub fn execute_join(&mut self, channel: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("JOIN {channel}"));
            println!("Joined {channel}");
        })
    }


    pub fn execute_part(&mut self, channel: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("PART {channel}"));
            println!("You have left {channel}");
        })
    }


    pub fn execute_privmsg(&mut self, target: String, message: String) {
        let nick = self.nick.clone();

        self.with_stream(|stream| {
            Self::send_command(stream, format!("PRIVMSG {target} :{message}"));
            println!("<{}> {}: {}", nick, target , message);
        })
    }


    pub fn execute_notice(&mut self, target: String, message: String) {
        let nick = self.nick.clone();

        self.with_stream(|stream| {
            Self::send_command(stream, format!("NOTICE {target} :{message}"));
            println!("<{}> {}: {}", nick, target , message);
        })
    }


    pub fn execute_quit(&mut self, message: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("QUIT {message}"));
            println!("Quitting...");
        })
    }

    pub fn execute_mode(&mut self, target: String, mode: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("MODE {target} {mode}"));
        })
    }

    pub fn execute_topic(&mut self, channel: String, topic: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("TOPIC {channel} {topic}"));
            println!("Change {channel} topic to '{topic}'");
        })
    }

    pub fn execute_who(&mut self, target: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("WHO {target}"));
            println!("List: ");
        })
    }

    pub fn execute_whois(&mut self, targets: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("WHOIS {targets}"));
            println!("List: ");
        })
    }

    pub fn execute_list(&mut self) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("LIST"));
            println!("List: ");
        })
    }

    pub fn execute_kick(&mut self, channel: String, user: String, reason: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("KICK {channel} {user} :{reason}"));
        })
    }

    pub fn execute_invite(&mut self, nick: String, channel: String) {
        self.with_stream(|stream| {
            Self::send_command(stream, format!("INVITE {nick} {channel}"));
        })
    }
}