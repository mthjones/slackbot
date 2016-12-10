use slack::{Error, RtmClient, User};

/// The sender of a command to the bot.
pub struct Sender<'a> {
    /// A writable Slack channel that the command came from. Can be used to respond on the same
    /// channel.
    channel_writer: ChannelWriter<'a>,

    /// The user that sent the command.
    pub user: User
}

impl<'a> Sender<'a> {
    pub fn new<S: Into<String>>(client: &'a mut RtmClient, channel_id: S, user: User) -> Self {
        let channel_writer = ChannelWriter::new(channel_id, client);
        Sender {
            channel_writer: channel_writer,
            user: user
        }
    }

    /// Send a message to the channel that the message came from.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slackbot::{SlackBot, Sender};
    /// # let mut my_bot = SlackBot::new("bot", "YOUR_API_TOKEN");
    /// # my_bot.on("say-hello", Box::new(|sender: &mut Sender, args: &Vec<String>| {
    /// sender.respond_in_channel("Hello, world!");
    /// # }));
    /// ```
    pub fn respond_in_channel<S: Into<String>>(&mut self, message: S) -> Result<isize, Error> {
        self.channel_writer.write(message)
    }
}

struct ChannelWriter<'a> {
    channel_id: String,
    client: &'a RtmClient
}

impl<'a> ChannelWriter<'a> {
    fn new<S: Into<String>>(channel_id: S, client: &'a RtmClient) -> Self {
        ChannelWriter {
            channel_id: channel_id.into(),
            client: client
        }
    }

    fn write<S: Into<String>>(&mut self, message: S) -> Result<isize, Error> {
        self.client.send_message(&self.channel_id[..], &message.into()[..])
    }
}
