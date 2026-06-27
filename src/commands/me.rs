use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use weechat::{
    buffer::Buffer,
    hooks::{Command, CommandCallback, CommandSettings},
    Weechat, Args
};

use crate::Servers;

pub struct MeCommand {
    servers: Servers,
}

impl MeCommand {
    pub const DESCRIPTION: &'static str = "Send a /me action to the current channel";

    pub fn create(servers: &Servers) -> Result<Command, ()> {
        let settings = CommandSettings::new("me")
            .description(Self::DESCRIPTION)
            .add_argument("<message>")
            .arguments_description(
                "message: Message to send"
            );
        Command::new(
            settings,
            MeCommand {
                servers: servers.clone(),
            },
        )
    }
}

impl CommandCallback for MeCommand {
    fn callback(&mut self, _: &Weechat, buffer: &Buffer, arguments: Args) {
        if let Some(room) = self.servers.find_room(buffer) {
            let msg = arguments.skip(1).map(|arg| arg.to_string()).collect::<Vec<String>>();

            self.servers.runtime().block_on(room.send_message(
                RoomMessageEventContent::emote_plain(
                    msg.join(" ")
                ),
            ));
        }
    }
}
