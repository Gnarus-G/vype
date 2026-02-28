use std::sync::mpsc::Sender;
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
}

impl Dbusservice {
    pub fn new(msg_tx: Sender<DbusMsg>) -> Self {
        Self { msg_tx }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let tx = self.msg_tx.clone();

        thread::spawn(move || {
            run_dbus_service(tx);
        });

        Ok(())
    }
}

struct Recorder {
    msg_tx: Sender<DbusMsg>,
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
}

fn run_dbus_service(tx: Sender<DbusMsg>) {
    let recorder = Recorder { msg_tx: tx };

    let result = zbus::blocking::connection::Builder::session()
        .and_then(|b| b.name(SERVICE_NAME))
        .and_then(|b| b.serve_at(OBJECT_PATH, recorder))
        .and_then(|b| b.build());

    match result {
        Ok(_) => {}
        Err(e) => {
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
