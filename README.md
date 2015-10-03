# slackbot
A higher level interface for creating Slack bots in Rust.

[![Crate][crates-badge]][crates-href] [![Build Status][travis-badge]][travis-href]

## Documentation

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
slackbot = "*"
```

and this to your crate root:

```rust
extern crate slackbot;
```

## Example
Here's a simple example to show how to make a bot that just echoes back what was said to it:

```rust
extern crate slackbot;

use slackbot::{SlackBot, Sender};

fn main() {
    let mut echo_bot = SlackBot::new("bot", "BOT_API_TOKEN");

    echo_bot.on("echo", Box::new(|sender: &mut Sender, args: &Vec<String>| {
        if args.len() > 0 {
            sender.respond_in_channel(args.join(" ")).unwrap();
        } else {
            sender.respond_in_channel("echo echo echo").unwrap();
        }
    }));

    echo_bot.run().unwrap();
}
```

## License
`slackbot` is distributed under the [MIT License](./LICENSE).

[crates-badge]: https://img.shields.io/crates/v/slackbot.svg
[crates-href]: https://crates.io/crates/slackbot
[travis-badge]: https://travis-ci.org/mthjones/slackbot.svg?branch=master
[travis-href]: https://travis-ci.org/mthjones/slackbot
