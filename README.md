# NETCONF client playground

This is a playground project dedicated to create a NETCONF client with core functionality & good usability implemented in [Rust](https://www.rust-lang.org/).

Workspace is currently separated into two crates:
- `netconf-client` - library providing the core NETCONF protocol functionality to be used as needed in other apps/user interfaces
- `netconf-cli` - REPL app for execution of typical NETCONF commands & running basic use-case NETCONF sessions

# Beware

All the codebase is considered to be "alpha" stage, giving no guarantees of stability or fitness for use!
Any component and/or APIs can change without any backward compatibility considerations.
There is currently no explicit versioning for any of the components.

# Building

This project tries to be a typical [Rust](https://www.rust-lang.org/) workspace. *Rustaceans* supposedly know what to do, others may need to install Rust toolchain to build the `netconf-cli` binary for the above described usage.

**TL;DR**:
- [install](https://www.rust-lang.org/tools/install) rust
- checkout this git repository
- run "`cargo build --release`" in the checked out project dir
- locate created binary file inside the `target/` subdirectory of your platform (win/linux/mac) -> move/copy wherever need, and execute (no arguments)

# Running

`netconf-cli` binary is the one & only one file needed to run the NETCONF REPL.
It creates extra file with history of executed commands: `netconf-cli-history.txt` in the current working directory.

File can be safely deleted as needed, resulting in loss of the command execution history.

Beware! Please note this file includes all the commands executed from REPL, including any passwords used as parameters for logging into NETCONF API of target devices.
