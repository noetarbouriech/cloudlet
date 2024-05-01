use super::runner::Runner;
use crate::agent::{self, ExecuteRequest, ExecuteResponse, SignalRequest};
use agent::workload_runner_server::WorkloadRunner;
use std::{process, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response};

type Result<T> = std::result::Result<Response<T>, tonic::Status>;

pub struct WorkloadRunnerService {
    runner: Arc<Mutex<Runner>>,
}

impl WorkloadRunnerService {
    pub fn new(runner: Runner) -> Self {
        WorkloadRunnerService {
            runner: Arc::new(Mutex::new(runner)),
        }
    }
}

#[tonic::async_trait]
impl WorkloadRunner for WorkloadRunnerService {
    type ExecuteStream = ReceiverStream<std::result::Result<ExecuteResponse, tonic::Status>>;

    async fn execute(&self, _: Request<ExecuteRequest>) -> Result<Self::ExecuteStream> {
        // We assume there's only one request at a time
        let runner = self
            .runner
            .try_lock()
            .map_err(|e| tonic::Status::unavailable(format!("Runner is busy: {:?}", e)))?;

        let mut run_rx = runner
            .run()
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(output) = run_rx.recv().await {
                println!("Sending to the gRPC client: {}", output);
                let _ = tx.send(Ok(ExecuteResponse { output })).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn signal(&self, _: Request<SignalRequest>) -> Result<()> {
        process::exit(0);
    }
}
