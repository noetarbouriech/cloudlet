use std::sync::mpsc;

use crate::{
    agents::{rust, Agent, AgentOutput, Language},
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
    rx: mpsc::Receiver<String>,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = mpsc::channel();

        let agent: Box<dyn Agent + Sync + Send> = match config.language {
            Language::Rust => Box::new(rust::RustAgent::from(config.clone()).with_tx(tx)),
            #[cfg(feature = "debug-agent")]
            Language::Debug => Box::new(debug::DebugAgent::from(config.clone())),
        };

        Self { config, agent, rx }
    }

    pub fn run(&self) -> AgentResult<()> {
        let result = match self.config.action {
            Action::Prepare => self.agent.prepare()?,
            Action::Run => self.agent.run()?,
            Action::PrepareAndRun => {
                let res = self.agent.prepare()?;
                println!("Prepare result {:?}", res);
                self.agent.run()?;
                println!("Run result {:?}", res);
            }
        };

        loop {
            match self.rx.try_recv() {
                Ok(output) => {
                    println!("runner: {:?}", output);
                    println!("{}", output);
                }
                Err(e) => {
                    // handle error
                    println!("Error in receiving message: {}", e);
                    break;
                }
            }
        }
        while let Ok(output) = self.rx.try_recv() {
            println!("runner: {:?}", output);
        }

        Ok(())
    }
}
