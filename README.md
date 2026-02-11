# Atto
This is a small text editor built in Rust. It's job is to simply be an extremely simple and lightweight editor to edit files on the fly.

## Install
To install just input the following command:

`cargo install --git https://github.com/ElijahF-robotics/atto`

## Instructions
To use just run `atto filename`. If a filename is not supplied then atto will make a file without a name.
Once in the program simply type to use it. **Note** - tab does not currently work.

There are also some commands available:
- ctrl-q : quit the program
- ctrl-left arrow : go to beginning of line
- ctrl-right arrow : go to end of line
- ctrl-down arrow : go to bottom of file
- ctrl-up arrow : go to top of file
- ctrl-s : enter save mode

Once in save mode you'll be able to edit the file's name. From there you have a few commands available:
- enter - save the file and exit the program
- escape - exit save mode and go back to editing

Importantly, if you escape it does NOT save the file. So, if you press escape then do ctrl-q, you won't have a file. Escape simply saves the filename and lets you go back to editing.

**Note for Mac users** - some of the commands, namely the ctrl-arrow commands, are already taken by system. Due to this you may need to disable some system shortcuts for those to work properly.

## Bugs
If you have any problems feel free to leave a pull request and I'll work on fixing it.
