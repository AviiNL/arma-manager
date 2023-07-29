use std::{ffi::OsString, ops::Deref, path::PathBuf, sync::Arc};

use sysinfo::Pid;
use sysinfo::{ProcessExt, SystemExt};
use tokio::sync::{
    mpsc::{error::TryRecvError, *},
    RwLock,
};

pub fn is_running(process_name: &str) -> bool {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for _ in system.processes_by_name(process_name) {
        return true;
    }

    false
}

pub fn kill(process_name: &str) {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for process in system.processes_by_name(process_name) {
        process.kill();
    }
}

enum ControlMessage {
    Stop,
    Kill,
}

enum ProcessMessage {
    Output(String),
    Finished,
}

#[derive(Clone)]
pub struct ProcessControls {
    inner: Arc<InnerProcessControls>,
}

impl From<InnerProcessControls> for ProcessControls {
    fn from(inner: InnerProcessControls) -> Self {
        Self { inner: Arc::new(inner) }
    }
}

impl Deref for ProcessControls {
    type Target = InnerProcessControls;

    fn deref(&self) -> &Self::Target {
        use std::borrow::Borrow;
        self.inner.borrow()
    }
}

unsafe impl Send for ProcessControls {}
unsafe impl Sync for ProcessControls {}

pub struct InnerProcessControls {
    tx: UnboundedSender<ControlMessage>,
    rx: RwLock<UnboundedReceiver<ProcessMessage>>,
}

#[allow(unused)]
impl InnerProcessControls {
    fn new(tx: UnboundedSender<ControlMessage>, rx: UnboundedReceiver<ProcessMessage>) -> Self {
        Self {
            tx,
            rx: RwLock::new(rx),
        }
    }

    pub fn stop(&self) {
        let _ = self.tx.send(ControlMessage::Stop);
    }

    pub fn kill(&self) {
        let _ = self.tx.send(ControlMessage::Kill);
    }
}

impl ProcessControls {
    pub async fn next(&self) -> anyhow::Result<Option<String>> {
        let mut lock = self.rx.write().await;

        match lock.try_recv() {
            Ok(ProcessMessage::Output(output)) => Ok(Some(output)),
            Ok(ProcessMessage::Finished) => Ok(None),
            Err(TryRecvError::Empty) => Err(anyhow::anyhow!("No message available")), // this needs to get ignored.
            Err(TryRecvError::Disconnected) => Ok(None),
        }
    }

    pub async fn wait(&self) {
        loop {
            match self.next().await {
                Ok(Some(_)) => {}
                Ok(None) => {
                    break;
                }
                Err(_) => {}
            }
        }
    }
}

pub struct Process {
    program: PathBuf,
    arguments: Vec<String>,
    log_file: Option<PathBuf>, // should be anything that implements Write
}

impl Process {
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            arguments: vec![],
            log_file: None,
        }
    }

    pub fn log_to_file(&mut self, file: PathBuf) {
        self.log_file = Some(file);
    }

    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Process {
        self.arguments.push(arg.into());
        self
    }
    #[cfg(not(target_os = "windows"))]
    pub fn start(self) -> Result<ProcessControls, ProcessError> {
        tracing::error!("Platform not supported.");

        let (tx, rx) = unbounded_channel::<ControlMessage>();
        let (ptx, prx) = unbounded_channel::<ProcessMessage>();

        let _ = ptx.send(ProcessMessage::Finished);

        Ok(InnerProcessControls::new(tx, prx).into())
    }

    #[cfg(target_os = "windows")]
    pub fn start(self) -> Result<ProcessControls, ProcessError> {
        use winptyrs::{AgentConfig, MouseMode, PTYArgs, PTYBackend, PTY};

        let (tx, mut rx) = unbounded_channel::<ControlMessage>();
        let (ptx, prx) = unbounded_channel::<ProcessMessage>();

        let program = self.program.clone();
        let arguments = self.arguments.join(" ");

        let pty_args = PTYArgs {
            cols: 512,
            rows: 25,
            mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
            timeout: 500,
            agent_config: AgentConfig::WINPTY_FLAG_PLAIN_OUTPUT,
        };

        let mut pty = PTY::new_with_backend(&pty_args, PTYBackend::ConPTY)?;

        let cwd = std::env::current_dir().unwrap();

        tracing::info!("Starting process: {} {}", program.display(), arguments);

        pty.spawn(program.into(), Some(arguments.into()), Some(cwd.into()), None)?;

        self.truncate_log();

        tokio::spawn(async move {
            let pid = pty.get_pid();
            while pty.is_alive().unwrap() {
                // check if there are signals we need to process
                if let Ok(message) = rx.try_recv() {
                    match message {
                        ControlMessage::Stop => {
                            // write ctrl+c to the process
                            if let Err(_) = pty.write("".into()) {
                                break;
                            }
                        }
                        ControlMessage::Kill => {
                            // get the process handle
                            let pid = pty.get_pid();
                            let s = sysinfo::System::new_all();
                            if let Some(process) = s.process(Pid::from(pid as usize)) {
                                // kill the process
                                process.kill();
                                // sleep for 100ms
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                if let Err(_) = pty.write("".into()) {
                                    break;
                                }
                            }
                        }
                    }
                }

                // check if there is data to read
                if let Ok(output) = pty.read(512, false) {
                    let output = output.into_string().unwrap();
                    let _ = ptx.send(ProcessMessage::Output(output.clone().trim().to_string()));
                    self.log(output.as_bytes());
                } else {
                    break;
                }

                // Check if the process is still alive
                if sysinfo::System::new_all().process(Pid::from(pid as usize)).is_none() {
                    break;
                }
            }
            let _ = ptx.send(ProcessMessage::Finished);
        });

        Ok(InnerProcessControls::new(tx, prx).into())
    }
}

/// Logging
impl Process {
    fn log(&self, msg: &[u8]) {
        let Some(filepath) = &self.log_file else {
            return;
        };

        use std::io::Write;
        // open file for append
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap();

        file.write_all(msg).unwrap();
    }

    fn truncate_log(&self) {
        let Some(filepath) = &self.log_file else {
            return;
        };
        // open file for append
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(filepath)
            .unwrap();

        file.set_len(0).unwrap();
    }
}

#[derive(Debug)]
pub enum ProcessError {
    Message(String),
}

impl From<OsString> for ProcessError {
    fn from(e: OsString) -> Self {
        Self::Message(e.into_string().unwrap())
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ProcessError {}
