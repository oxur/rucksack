// use std::time::Duration;

use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient, System, SystemRunner};
use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::input::Config;

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

pub struct Daemon {
    pub app: Option<App>,
    recipient: Recipient<Command>,
}

impl Daemon {
    pub fn new(app: Option<App>, recipient: Recipient<Command>) -> Daemon {
        Daemon { app, recipient }
    }
}

impl Actor for Daemon {
    type Context = Context<Self>;
}

impl Handler<Command> for Daemon {
    // type Result = Vec<u8>;
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Command, _ctx: &mut actix::Context<Self>) -> Result<()> {
        println!("Got msg {msg:?} from {:?}", self.recipient);

        // wait 100 nanoseconds
        // ctx.run_later(Duration::new(0, 100), move |act, _| {
        //     act.recipient
        //         .do_send(Command::new("notify started".to_string(), None));
        // });
        Ok(())
    }
}

pub fn start(cfg: Config, matches: &ArgMatches) -> Result<SystemRunner> {
    let system = System::new();
    let app = App::new(cfg, matches)?;
    system.block_on(async {
        // If we want, we can postpone actor creation and send the
        // recipient a message first:
        Daemon::create(|ctx| {
            // // Get the address of the first actor
            let addr = ctx.address();
            // // Create a second actor
            // let addr2 = Daemon::new(None, addr.recipient()).start();

            // // Send a message
            // addr2.do_send(Command::new("notify started".to_string(), None));

            // // now we can finally create first actor
            // Daemon::new(Some(app), addr2.recipient())
            Daemon::new(Some(app), addr.recipient())
        });
    });
    log::info!("Starting rucksack daemon ...");
    Ok(system)
}
