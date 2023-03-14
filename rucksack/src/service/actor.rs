use actix::{Actor, AsyncContext, Context, Handler, Recipient, System, SystemRunner};
use anyhow::Result;

use crate::app::App;

use super::protocol::Command;

pub struct Commander {
    pub app: App,
    recipient: Recipient<Command>,
}

impl Commander {
    pub fn start(app: App) -> Result<SystemRunner> {
        log::info!("Starting rucksack daemon ...");
        let system = System::new();
        system.block_on(async {
            Commander::create(|ctx| Commander {
                app,
                recipient: ctx.address().recipient(),
            });
        });
        log::debug!("Starting Actix system runner ...");
        Ok(system)
    }

    pub fn stop() {
        log::info!("Stopping rucksack daemon ...");
        System::current().stop()
    }
}

impl Actor for Commander {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        log::info!("Commander has started at {:?}", ctx.address());
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        log::info!("Commander has stopped");
    }
}

impl Handler<Command> for Commander {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Command, _ctx: &mut actix::Context<Self>) -> Result<()> {
        log::info!("Got msg {msg:?} from {:?}", self.recipient);
        // TODO: add a command dispatch
        Ok(())
    }
}
