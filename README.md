# DiscordNetBot
A Discord JSON invoked bot over TCP

Index:
  - 1.0 [Introduction](https://github/Skarlett/DiscordNetBot#Introduction)
  - 1.1 [Design](https://github/Skarlett/DiscordNetBot#Design)
  - 2.0 [Installation](https://github/Skarlett/DiscordNetBot#Installation)
  - 2.1 [Dependencies](https://github/Skarlett/DiscordNetBot#Dependencies)
  - 3.0 [Using API](https://github/Skarlett/DiscordNetBot#API)
  - 3.1 [Extending API](https://github/Skarlett/DiscordNetBot#Extending)



## Introduction

This is a bot used for discord. It **doesn't currently have any commands facing towards the discord client**, but instead has an open TCP listener running, and waits for commands to sent from other nodes in the network. It processes these requests in `JSON` from a threadpool, that will reply with a `JSON` response, mimicing your input + the response data.

### Design
This bot wasn't made to conquer the other bots on functionality. It was designed to have a near minimum runtime, allowing it to use the CPU completely through thread pools, and leaving as small as a memory mark as possible. There are other things I would like to add to this bot, but have not yet.

    [ ] Logging
    [ ] Descriptions and arguments in help menu
    [ ] Allow new lines in json request

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

## API
The API accepts parameters as a strict data stream represented in JSON and defined by ending it with `0xA`/`\n` (new line break). It will normally sit on **port 9449**, unless configured differently. The API will close the connection after the invoked action is done.

    {"action":"info","arguments":[]}\n
    
The API responds with mimicing your input, and telling you if it worked.

    {"action":"info","arguments":[], "status":"OK", "response": {...}}
    
### Extending
The API functionality is manifested in `src/api/mod.rs`, by adding closures to the `API!` macro in that file, you can extend the functionality of the API.
