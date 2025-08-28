# BareIRC

Another IRC client!  
Written in Rust...

![Client test](/assets/image.png)

&nbsp;

# Help
To connect to any network you must first set a nickname, a username, and a realname.

&nbsp;

# Commands

Commands are case insensitive and start with the `/` character:
+   `/nick <nickname>`: Set personal nickname.

+   `/user <username> <realname>`: Set personal username and realname.

+   `/server <hostname>:<port>`: Join a network, requires to have **nickname**, **username**, and **realname** already set.

+   `/join #<server>`: Join a server, all server must start with an hash sign.

+   `/privmsg #<target> <message>`: Send a \<message> to \<target>.

+   `/notice #<target> <message>`: Send a \<message> to \<target>.

+   `/part #<server>`: Leave the specified \<server>.

+   `/quit #<reason>`: Leave the network, \<reason> is mandatory.

+   `/topic #<channel> #<topic>`: Changes the topic for the specified server (channel).

+   `/who <target>`: Return a list of users who match \<target>.

+   `/whois <users>`: Print infos about the comma-separated list of users.

+   `/list`: List all the servers in the current network.

+   `/kick <channel> <target> :<reason>`: Kick \<target> from \<channel> with the given \<reason>.

+   `/invite <nickname> <channel>`: Invite \<nickname> to \<channel>.


Client side commands:
+   `/exit`: Exits the client. Alternatively, pressing the \<Esc> button does the same.

+   `/help`: List all the available commands.

&nbsp;

# Keybinds
+   `Esc`: Exits the client. Alternatively, the `/exit` commands does the same.

+   `Up`: Moves the chat up.

+   `Down`: Moves the chat down.

&nbsp;

# Changelog
+ ***3.0***
    + File structure changes
    + Bug Fixes
    + TUI changes

+ ***0.2.5***
    + Auto scroll feature
    + Bug Fixes
    + Added Changelog

+ ***0.2***
    + TUI improvement
    + Bug Fixes

+ ***0.1***
    + Release


# To-Do
+ Proper TUI (colored usernames...).