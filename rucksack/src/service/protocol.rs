use actix::Message;

type CommandData = Vec<u8>;

#[derive(Message, Debug)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Command {
    pub cmd: String,
    pub data: Option<CommandData>,
}

impl Command {
    pub fn new(cmd: String, data: Option<CommandData>) -> Command {
        Command { cmd, data }
    }
}
