//! slackbot is here to make creating your own Slack bot easy. It provides a relatively high-level
//! interface for creating Slack bots.
//!
//! # Examples
//!
//! Creates a bot that will respond to a message like `!bot echo Hello world!` with `Hello world!`
//! for any channels it is in.
//!
//! ```rust,no_run
//! extern crate slackbot;
//!
//! use slackbot::{SlackBot, Sender};
//!
//! fn main() {
//!     let mut echo_bot = SlackBot::new("bot", "BOT_API_TOKEN");
//!
//!     echo_bot.on("echo", Box::new(|sender: &mut Sender, args: &Vec<String>| {
//!         if args.len() > 0 {
//!             sender.respond_in_channel(args.join(" ")).unwrap();
//!         } else {
//!             sender.respond_in_channel("echo echo echo").unwrap();
//!         }
//!     }));
//!
//!     echo_bot.run().unwrap();
//! }
//! ```

extern crate slack;
extern crate serde_json;

use std::collections::HashMap;
use slack::RtmClient;

mod event_handler;
mod sender;

use event_handler::SlackBotEventHandler;
pub use sender::Sender;

/// The bot that handles commands and communication with Slack.
pub struct SlackBot {
    name: String,
    token: String,
    handlers: HashMap<String, Box<CommandHandler>>
}

impl SlackBot {
    /// Create a new bot to serve your team!
    ///
    /// # Examples
    ///
    /// ```
    /// use slackbot::SlackBot;
    ///
    /// let mut my_bot = SlackBot::new("bot", "YOUR_API_TOKEN");
    /// ```
    pub fn new<A,B>(name: A, token: B) -> Self
        where A: Into<String>, B: Into<String> {

        SlackBot {
            name: name.into(),
            token: token.into(),
            handlers: HashMap::new()
        }
    }

    /// Tell your bot what to do when it sees a command.
    ///
    /// The handler can be your own type that implements `CommandHandler`, but most simple cases
    /// can be covered by a simple closure.
    ///
    /// # Examples
    ///
    /// With a simple closure:
    ///
    /// ```
    /// # use slackbot::{SlackBot, Sender};
    /// # let mut my_bot = SlackBot::new("bot", "YOUR_API_TOKEN");
    /// my_bot.on("say-hello", Box::new(|sender: &mut Sender, args: &Vec<String>| {
    ///     sender.respond_in_channel("Hello, world!");
    /// }));
    /// ```
    ///
    /// With an implemented CommandHandler:
    ///
    /// ```
    /// # use slackbot::{SlackBot, Sender, CommandHandler};
    /// # let mut my_bot = SlackBot::new("bot", "YOUR_API_TOKEN");
    /// struct SayHelloCommandHandler;
    ///
    /// impl CommandHandler for SayHelloCommandHandler {
    ///     fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
    ///         sender.respond_in_channel("Hello, world!");
    ///     }
    /// }
    ///
    /// my_bot.on("say-hello", Box::new(SayHelloCommandHandler));
    /// ```
    pub fn on<S: Into<String>>(&mut self, command_name: S, handler: Box<CommandHandler>) {
        self.handlers.insert(command_name.into(), handler);
    }

    /// Tell your bot to start pulling its weight!
    ///
    /// # Examples
    ///
    /// ```
    /// # use slackbot::{SlackBot, Sender};
    /// # let mut my_bot = SlackBot::new("bot", "YOUR_API_TOKEN");
    /// match my_bot.run() {
    ///     Ok(()) => println!("Bot shut down successfully. Goodbye world!"),
    ///     Err(err) => println!("Bot crashed. Error message: {}", err)
    /// };
    /// ```
    pub fn run(&mut self) -> Result<(), String> {
        let mut client = RtmClient::new(&self.token[..]);
        let mut handler = SlackBotEventHandler::new(&self.name[..], &mut self.handlers);

        client.login_and_run(&mut handler)
    }
}

/// A trait implemented by types that can handle commands.
///
/// # Examples
///
/// ```
/// # use slackbot::{Sender, CommandHandler};
/// struct SayHelloCommandHandler;
///
/// impl CommandHandler for SayHelloCommandHandler {
///     fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
///         sender.respond_in_channel("Hello, world!");
///     }
/// }
/// ```
pub trait CommandHandler {
    /// Handle the command.
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>);
}

impl<F> CommandHandler for F where F: FnMut(&mut Sender, &Vec<String>) {
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
        self(sender, args);
    }
}
