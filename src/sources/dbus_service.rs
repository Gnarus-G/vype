use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SERVICE_NAME: &str = "tech.bytin.vype";
const OBJECT_PATH: &str = "/tech/bytin/vype";
const INTERFACE_NAME: &str = "tech.bytin.vype.Recorder";

fn dbg(s: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/vype-dbus.log")
    {
        let _ = writeln!(f, "{}", s);
    }
    eprintln!("{}", s);
}

#[derive(Clone, Copy, Debug)]
pub enum DbusMsg {
    StartRecording,
    StopRecording,
    ToggleRecording,
}

pub struct Dbusservice {
    msg_tx: std::sync::mpsc::Sender<DbusMsg>,
    recording: Arc<AtomicBool>,
}

impl Dbusservice {
    pub fn new(msg_tx: std::sync::mpsc::Sender<DbusMsg>) -> Self {
        Self {
            msg_tx,
            recording: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn recording_flag(&self) -> Arc<AtomicBool> {
        self.recording.clone()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let tx = self.msg_tx.clone();
        let recording = self.recording.clone();

        thread::spawn(move || {
            run_dbus_service(tx, recording);
        });

        Ok(())
    }
}

struct Recorder {
    recording: Arc<AtomicBool>,
    msg_tx: std::sync::mpsc::Sender<DbusMsg>,
}

#[zbus::interface(name = "tech.bytin.vype.Recorder")]
impl Recorder {
    fn start_recording(&self) -> bool {
        let _ = self.msg_tx.send(DbusMsg::StartRecording);
        true
    }

    fn stop_recording(&self) -> bool {
        let _ = self.msg_tx.send(DbusMsg::StopRecording);
        true
    }

    fn toggle_recording(&self) -> bool {
        let _ = self.msg_tx.send(DbusMsg::ToggleRecording);
        true
    }

    #[zbus(property)]
    fn is_recording(&self) -> bool {
        self.recording.load(Ordering::SeqCst)
    }
}

fn run_dbus_service(tx: std::sync::mpsc::Sender<DbusMsg>, recording: Arc<AtomicBool>) {
    dbg("Starting D-Bus service...");

    let recorder = Recorder {
        recording: recording.clone(),
        msg_tx: tx,
    };

    dbg("Creating session builder...");

    let result = zbus::blocking::connection::Builder::session()
        .and_then(|b| {
            dbg("Got session builder, requesting name...");
            b.name(SERVICE_NAME)
        })
        .and_then(|b| {
            dbg("Got name, serving at object path...");
            b.serve_at(OBJECT_PATH, recorder)
        })
        .and_then(|b| {
            dbg("Built connection");
            b.build()
        });

    dbg(&format!("Result: {:?}", result));

    let connection = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("D-Bus connection error: {}", e);
            log::error!("Failed to create D-Bus connection: {}", e);
            return;
        }
    };

    log::info!(
        "D-Bus service running. Access via: busctl call {} {} {} ToggleRecording",
        SERVICE_NAME,
        OBJECT_PATH,
        INTERFACE_NAME
    );

    loop {
        thread::sleep(Duration::from_millis(500));
    }
}
