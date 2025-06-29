# IRC Client

Another IRC client!
Written in Rust.

---

# Commands

Commands are case-insensitive.  
This means that **/join**, **/Join**, **/JOIN** (and all the other combinations) are all valid command.

+   `/nick <nickname>`: set personal nickname.
+   `/user <username> <realname>`: set personal username and realname.
+   `/server <hostname>:<port>`: join a network, requires to have **nickname**, **username**, and **realname** already set.
+   `/join #<server>`: join a server, all server must start with an hash sign.
