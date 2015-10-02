extern crate slack;
extern crate serde_json;

mod event_handler;

use std::collections::HashMap;

use slack::{RtmClient,User};
use event_handler::SlackBotEventHandler;

pub struct SlackBot {
    name: String,
    token: String,
    handlers: HashMap<String, Box<CommandHandler>>
}

impl SlackBot {
    pub fn new<A,B>(name: A, token: B) -> Self
        where A: Into<String>, B: Into<String> {

        SlackBot {
            name: name.into(),
            token: token.into(),
            handlers: HashMap::new()
        }
    }

    pub fn on<S: Into<String>>(&mut self, command_name: S, handler: Box<CommandHandler>) {
        self.handlers.insert(command_name.into(), handler);
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut client = RtmClient::new(&self.token[..]);
        let mut handler = SlackBotEventHandler::new(&self.name[..], &mut self.handlers);

        client.login_and_run(&mut handler)
    }
}

pub struct Sender<'a> {
    pub channel: ChannelWriter<'a>,
    pub user: User
}

pub trait CommandHandler {
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>);
}

impl<F> CommandHandler for F where F: FnMut(&mut Sender, &Vec<String>) {
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
        self(sender, args);
    }
}

pub struct ChannelWriter<'a> {
    channel_id: String,
    client: &'a mut RtmClient
}

impl<'a> ChannelWriter<'a> {
    pub fn new<S: Into<String>>(channel: S, client: &'a mut RtmClient) -> Self {
        ChannelWriter {
            channel_id: channel.into(),
            client: client
        }
    }

    pub fn write<S: Into<String>>(&mut self, message: S) -> Result<(), String> {
        self.client.post_message(&self.channel_id[..], &message.into()[..], None).map(|_| ())
    }
}
