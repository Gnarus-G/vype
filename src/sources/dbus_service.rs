use std::sync::mpsc::{channel, Sender};
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
        let (result_tx, result_rx) = channel::<anyhow::Result<()>>();

        thread::spawn(move || {
            run_dbus_service(tx, result_tx);
        });

        result_rx.recv()??;
        Ok(())
    }
}

struct Recorder {
    msg_tx: Sender<DbusMsg>,
}

#[zbus::interface(name = "tech.bytin.vype.Recorder")]
impl Recorder {
    fn start_recording(&self) -> bool {
        self.msg_tx.send(DbusMsg::StartRecording).is_ok()
    }

    fn stop_recording(&self) -> bool {
        self.msg_tx.send(DbusMsg::StopRecording).is_ok()
    }

    fn toggle_recording(&self) -> bool {
        self.msg_tx.send(DbusMsg::ToggleRecording).is_ok()
    }
}

fn run_dbus_service(tx: Sender<DbusMsg>, result_tx: Sender<anyhow::Result<()>>) {
    let recorder = Recorder { msg_tx: tx };

    let result = zbus::blocking::connection::Builder::session()
        .and_then(|b| b.name(SERVICE_NAME))
        .and_then(|b| b.serve_at(OBJECT_PATH, recorder))
        .and_then(|b| b.build());

    match result {
        Ok(_) => {}
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

    loop {
        thread::sleep(Duration::from_millis(500));
    }
}
