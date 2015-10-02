use std::collections::HashMap;

use slack::{EventHandler,RtmClient};
use serde_json::{self, Value};

use super::{CommandHandler,ChannelWriter,Sender};

struct UserCommand {
    command: String,
    args: Vec<String>,
    user_id: String,
    channel: String
}

pub struct SlackBotEventHandler<'a> {
    bot_name: String,
    handlers: &'a mut HashMap<String, Box<CommandHandler>>
}

impl<'a> SlackBotEventHandler<'a> {
    pub fn new<S: Into<String>>(name: S, handlers: &'a mut HashMap<String, Box<CommandHandler>>) -> Self {
        SlackBotEventHandler {
            bot_name: name.into(),
            handlers: handlers
        }
    }

    // TODO: Replace lots of this with proper serde deserialization
    fn parse_json_to_command(bot_name: &str, json_str: &str) -> Option<UserCommand> {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let message = data.as_object().unwrap();

        if let Some(&Value::String(ref ty)) = message.get("type") {
            if ty == "message" {
                if let Some(&Value::String(ref text)) = message.get("text") {
                    let bang_command = "!".to_owned() + bot_name;
                    if text.starts_with(&bang_command[..]) {
                        let mut command_pieces = text.split_whitespace().skip(1);
                        let (command, args) = match command_pieces.next() {
                            Some(c) => (c, command_pieces.map(|arg| arg.to_owned()).collect::<Vec<_>>()),
                            None => ("help", vec![])
                        };

                        if let Some(&Value::String(ref user_id)) = message.get("user") {
                            if let Some(&Value::String(ref channel)) = message.get("channel") {
                                return Some(UserCommand {
                                    command: command.to_owned(),
                                    args: args,
                                    user_id: user_id.to_owned(),
                                    channel: channel.to_owned()
                                });
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

impl<'a> EventHandler for SlackBotEventHandler<'a> {
    fn on_receive(&mut self, cli: &mut RtmClient, json_str: &str) {
        if let Some(cmd) = Self::parse_json_to_command(&self.bot_name[..], json_str) {
            let user = cli.get_users().iter().find(|u| u.id == cmd.user_id).unwrap().clone();
            if let Some(handler) = self.handlers.get_mut(&cmd.command[..]) {
                let writer = ChannelWriter::new(cmd.channel, cli);
                let mut sender = Sender {
                    channel: writer,
                    user: user
                };
                handler.handle(&mut sender, &cmd.args);
            }

            println!("Got command: {}", cmd.command);
        }
    }

    fn on_ping(&mut self, _: &mut RtmClient) {}

    fn on_close(&mut self, _: &mut RtmClient) {}

    fn on_connect(&mut self, _: &mut RtmClient) {}
}
