//! Code execution.

use crate::markdown::{code::EvaluatorId, elements::Code};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
    process::{self, ChildStderr, ChildStdout, Stdio},
    sync::{Arc, Mutex},
    thread,
};
use tempfile::NamedTempFile;

/// Allows executing code.
pub(crate) struct CodeExecuter;

impl CodeExecuter {
    /// Execute a piece of code.
    pub(crate) fn execute(
        code: &Code,
        evaluators: &HashMap<String, Vec<String>>,
    ) -> Result<ExecutionHandle, CodeExecuteError> {
        if let Some(evaluator_id) = &code.attributes.evaluator_id {
            match evaluator_id {
                EvaluatorId::Shell => Self::execute_shell(&["sh".to_string()], &code.contents),
                EvaluatorId::Custom(evaluator_id) => {
                    if let Some(args) = evaluators.get(evaluator_id) {
                        Self::execute_shell(args, &code.contents)
                    } else {
                        Err(CodeExecuteError::EvaluatorNotDefined(evaluator_id.to_string()))
                    }
                }
            }
        } else {
            Err(CodeExecuteError::NotExecutableCode)
        }
    }

    fn execute_shell(args: &[String], code: &str) -> Result<ExecutionHandle, CodeExecuteError> {
        let mut output_file = NamedTempFile::new().map_err(CodeExecuteError::TempFile)?;
        output_file.write_all(code.as_bytes()).map_err(CodeExecuteError::TempFile)?;
        output_file.flush().map_err(CodeExecuteError::TempFile)?;
        let process_handle = process::Command::new("/usr/bin/env")
            .args(args)
            .arg(output_file.path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(CodeExecuteError::SpawnProcess)?;

        let state: Arc<Mutex<ExecutionState>> = Default::default();
        let reader_handle = ProcessReader::spawn(process_handle, state.clone(), output_file);
        let handle = ExecutionHandle { state, reader_handle };
        Ok(handle)
    }
}

/// An error during the execution of some code.
#[derive(thiserror::Error, Debug)]
pub(crate) enum CodeExecuteError {
    #[error("code is not marked for execution")]
    NotExecutableCode,

    #[error("error creating temporary file: {0}")]
    TempFile(io::Error),

    #[error("error spawning process: {0}")]
    SpawnProcess(io::Error),

    #[error("evaluator not defined: {0}")]
    EvaluatorNotDefined(String),
}

/// A handle for the execution of a piece of code.
#[derive(Debug)]
pub(crate) struct ExecutionHandle {
    state: Arc<Mutex<ExecutionState>>,
    #[allow(dead_code)]
    reader_handle: thread::JoinHandle<()>,
}

impl ExecutionHandle {
    /// Get the current state of the process.
    pub(crate) fn state(&self) -> ExecutionState {
        self.state.lock().unwrap().clone()
    }
}

/// Consumes the output of a process and stores it in a shared state.
struct ProcessReader {
    handle: process::Child,
    state: Arc<Mutex<ExecutionState>>,
    #[allow(dead_code)]
    file_handle: NamedTempFile,
}

impl ProcessReader {
    fn spawn(
        handle: process::Child,
        state: Arc<Mutex<ExecutionState>>,
        file_handle: NamedTempFile,
    ) -> thread::JoinHandle<()> {
        let reader = Self { handle, state, file_handle };
        thread::spawn(|| reader.run())
    }

    fn run(mut self) {
        let stdout = self.handle.stdout.take().expect("no stdout");
        let stdout = BufReader::new(stdout);
        let stderr = self.handle.stderr.take().expect("no stderr");
        let stderr = BufReader::new(stderr);
        let _ = Self::process_output(self.state.clone(), stdout, stderr);
        let success = self.handle.try_wait().ok().flatten().map(|s| s.success());
        let status = match success {
            Some(true) => ProcessStatus::Success,
            Some(false) => ProcessStatus::Failure,
            None => ProcessStatus::Running,
        };
        self.state.lock().unwrap().status = status;
    }

    fn process_output(
        state: Arc<Mutex<ExecutionState>>,
        stdout: BufReader<ChildStdout>,
        stderr: BufReader<ChildStderr>,
    ) -> io::Result<()> {
        for line in stdout.lines() {
            let line = line?;
            // TODO: consider not locking per line...
            state.lock().unwrap().output.push(line);
        }
        for line in stderr.lines() {
            let line = line?;
            // TODO: consider not locking per line...
            state.lock().unwrap().error_output.push(line);
        }
        Ok(())
    }
}

/// The state of the execution of a process.
#[derive(Clone, Default, Debug)]
pub(crate) struct ExecutionState {
    pub(crate) output: Vec<String>,
    pub(crate) error_output: Vec<String>,
    pub(crate) status: ProcessStatus,
}

/// The status of a process.
#[derive(Clone, Debug, Default)]
pub(crate) enum ProcessStatus {
    #[default]
    Running,
    Success,
    Failure,
}

impl ProcessStatus {
    /// Check whether the underlying process is finished.
    pub(crate) fn is_finished(&self) -> bool {
        matches!(self, ProcessStatus::Success | ProcessStatus::Failure)
    }
}
