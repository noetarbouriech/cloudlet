use tokio::sync::mpsc::Receiver;

use crate::{
    agents::{rust, Agent, Language},
    workload::config::Action,
    AgentResult,
};

#[cfg(feature = "debug-agent")]
use crate::agents::debug;

use super::config::Config;

/// Runner for a workload.
/// Will execute the workload based on the inner agent (language).
pub struct Runner {
    config: Config,
    agent: Box<dyn Agent + Sync + Send>,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let agent: Box<dyn Agent + Sync + Send> = match config.language {
            Language::Rust => Box::new(rust::RustAgent::from(config.clone())),
            #[cfg(feature = "debug-agent")]
            Language::Debug => Box::new(debug::DebugAgent::from(config.clone())),
        };

        Self { config, agent }
    }

    pub fn run(&self) -> AgentResult<Receiver<String>> {
        let rx = match self.config.action {
            Action::Prepare => self.agent.prepare()?,
            Action::Run => self.agent.run()?,
            Action::PrepareAndRun => {
                self.agent.prepare()?;
                self.agent.run()?
            }
        };

        Ok(rx)
    }
}
