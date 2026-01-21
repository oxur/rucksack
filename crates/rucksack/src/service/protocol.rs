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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new_with_data() {
        let cmd = Command::new("test".to_string(), Some(vec![1, 2, 3]));
        assert_eq!(cmd.cmd, "test");
        assert_eq!(cmd.data, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_command_new_without_data() {
        let cmd = Command::new("test".to_string(), None);
        assert_eq!(cmd.cmd, "test");
        assert_eq!(cmd.data, None);
    }
}
