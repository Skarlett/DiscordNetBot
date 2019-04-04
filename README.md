# DiscordNetBot
A Discord JSON invoked bot over TCP

Index:
  - 1.0 Introduction
  - 1.1 Goals and Design details
  - 2.0 Installation
  - 2.1 Dependencies
  - 2.1 Using API
  - 2.2 Extending API



## Introduction

This is a bot used for discord. It **doesn't currently have any commands facing towards the discord client**, but instead has an open TCP listener running, and waits for commands to sent from other nodes in the network. It processes these requests in `JSON` from a threadpool, that will reply with a `JSON` response, mimicing your input + the response data.

### Goals and Design details
This bot wasn't made to conquer the other bots on functionality. It was designed to have a near minimum runtime, allowing it to use the CPU completely through thread pools, and leaving as small as a memory mark as possible. There are other things I would like to add to this bot, but have not yet.

    [ ] Logging
    [ ] Descriptions and arguments in help menu

## Installation
```
git clone https://github.com/Skarlett/DiscordNetBot
cd DiscordNetBot
cargo build --release
```

### Dependencies
You will need `gcc`, `config-pkg`, `openssl` and `cargo` or `rustup` to compile this project.
```
# On Debian and Ubuntu
sudo apt-get install pkg-config libssl-dev
# On Arch Linux
sudo pacman -S openssl
# On Fedora
sudo dnf install openssl-devel
```
