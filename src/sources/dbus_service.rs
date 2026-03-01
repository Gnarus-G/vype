use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SERVICE_NAME: &str = "tech.bytin.vype";
const OBJECT_PATH: &str = "/tech/bytin/vype";
const INTERFACE_NAME: &str = "tech.bytin.vype.Recorder";

#[derive(Clone, Copy, Debug)]
pub enum DbusMsg {
    StartRecording,
    StopRecording,
    ToggleRecording,
}

pub struct Dbusservice {
    msg_tx: Sender<DbusMsg>,
    is_recording: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
}

impl Dbusservice {
    pub fn new(msg_tx: Sender<DbusMsg>, is_recording: Arc<AtomicBool>) -> Self {
        Self {
            msg_tx,
            is_recording,
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let tx = self.msg_tx.clone();
        let recording = self.is_recording.clone();
        let shutdown = self.shutdown.clone();
        let (result_tx, result_rx) = channel::<anyhow::Result<()>>();

        thread::spawn(move || {
            run_dbus_service(tx, recording, result_tx, shutdown);
        });

        result_rx
            .recv_timeout(Duration::from_secs(10))
            .map_err(|e| anyhow::anyhow!("D-Bus service startup timed out: {}", e))??;
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

struct Recorder {
    msg_tx: Sender<DbusMsg>,
    is_recording: Arc<AtomicBool>,
}

#[zbus::interface(name = "tech.bytin.vype.Recorder")]
impl Recorder {
    fn start_recording(&self) -> bool {
        if self.msg_tx.send(DbusMsg::StartRecording).is_ok() {
            true
        } else {
            false
        }
    }

    fn stop_recording(&self) -> bool {
        if self.msg_tx.send(DbusMsg::StopRecording).is_ok() {
            true
        } else {
            false
        }
    }

    fn toggle_recording(&self) -> bool {
        if self.msg_tx.send(DbusMsg::ToggleRecording).is_ok() {
            true
        } else {
            false
        }
    }

    #[zbus(property)]
    fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

fn run_dbus_service(
    tx: Sender<DbusMsg>,
    recording: Arc<AtomicBool>,
    result_tx: Sender<anyhow::Result<()>>,
    shutdown_rx: Arc<AtomicBool>,
) {
    let recorder = Recorder {
        msg_tx: tx,
        is_recording: recording,
    };

    let result = zbus::blocking::connection::Builder::session()
        .and_then(|b| b.name(SERVICE_NAME))
        .and_then(|b| b.serve_at(OBJECT_PATH, recorder))
        .and_then(|b| b.build());

    let _conn = match result {
        Ok(conn) => conn,
        Err(e) => {
            let err = anyhow::anyhow!("Failed to create D-Bus connection: {}", e);
            let _ = result_tx.send(Err(err));
            return;
        }
    };

    let _ = result_tx.send(Ok(()));

    log::info!(
        "D-Bus service running. Access via: busctl --user call {} {} {} ToggleRecording",
        SERVICE_NAME,
        OBJECT_PATH,
        INTERFACE_NAME
    );

    while !shutdown_rx.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    log::info!("D-Bus service shutting down");
}
