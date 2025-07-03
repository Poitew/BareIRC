# IRC Client

Another IRC client!
Written in Rust.

---
# Help
To connect to a network you first need to have set a nickname, username, and realname.
To do so, simply use the `NICK` command, and the `USER` command.

---
# Commands

Commands are case-insensitive.  
This means that **/join**, **/Join**, **/JOIN** (and all the other combinations) are all valid commands.

+   `/nick <nickname>`: set personal nickname.
+   `/user <username> <realname>`: set personal username and realname.
+   `/server <hostname>:<port>`: join a network, requires to have **nickname**, **username**, and **realname** already set.
+   `/join #<server>`: join a server, all server must start with an hash sign.
+   `/privmsg #<target> <message>`: send a message in the specified server, or to the specified user.
+   `/notice #<target> <message>`: send a message in the specified server, or to the specified user.
+   `/part #<server>`: leave the specified <server>.
+   `/quit #<reason>`: leave the network, <reason> is mandatory.
+   `/topic #<channel> #<topic>`: changes the topic for the specified server (channel).
+   `/who <target>`: return a list of users who match <target>.
+   `/whois <users>`: print infos about the comma-separated list of users.
+   `/list`: list all the servers in the current network.
+   `/kick <channel> <target> :<reason>`: kick <target> from <channel> with the given <reason>.
+   `/invite <nickname> <channel>`: invite <nickname> to <channel>.


Client side commands:
+   `/exit`: exits the client. Alternatively pressing the <Esc> button does the same.
+   `/help`: list all the available commands.

