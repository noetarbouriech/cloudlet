use tokio::sync::mpsc::Receiver;

use crate::AgentResult;
use serde::Deserialize;

#[cfg(feature = "debug-agent")]
pub mod debug;
pub mod rust;

#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub trait Agent {
    fn prepare(&self) -> AgentResult<Receiver<String>>;
    fn run(&self) -> AgentResult<Receiver<String>>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    Rust,
    #[cfg(feature = "debug-agent")]
    Debug,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            #[cfg(feature = "debug-agent")]
            Language::Debug => write!(f, "debug"),
        }
    }
}
