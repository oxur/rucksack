use actix::{Actor, AsyncContext, Context, Handler, Recipient, System, SystemRunner};
use anyhow::Result;

use crate::app::App;

use super::protocol::Command;

pub struct Commander {
    pub app: App,
    #[allow(dead_code)] // TODO: will be used when command dispatch is implemented
    recipient: Recipient<Command>,
}

impl Commander {
    pub fn start(app: App) -> Result<SystemRunner> {
        log::info!(operation = "start_daemon"; "Starting rucksack daemon");
        let system = System::new();
        system.block_on(async {
            Commander::create(|ctx| Commander {
                app,
                recipient: ctx.address().recipient(),
            });
        });
        log::debug!(operation = "start_runner"; "Starting Actix system runner");
        Ok(system)
    }

    pub fn stop() {
        log::info!(operation = "stop_daemon"; "Stopping rucksack daemon");
        System::current().stop()
    }
}

impl Actor for Commander {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        log::info!(operation = "start"; "Commander has started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        log::info!(operation = "stop"; "Commander has stopped");
    }
}

impl Handler<Command> for Commander {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Command, _ctx: &mut actix::Context<Self>) -> Result<()> {
        log::info!(operation = "handle"; "Got message");
        // TODO: add a command dispatch
        Ok(())
    }
}
