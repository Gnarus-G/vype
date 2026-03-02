use anyhow::Result;
use clap::{Parser, ValueEnum};
use iceoryx2::port::publisher::Publisher;
use iceoryx2::prelude::*;
use log::info;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use vype_shared::{PttEvent, PttEventType};

#[derive(Parser, Debug)]
#[command(name = "vypec")]
#[command(about = "Vype client - send control commands to daemon")]
struct Args {
    #[arg(value_enum, default_value_t = Command::Toggle)]
    command: Command,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Command {
    Start,
    Stop,
    Toggle,
    Partial,
}

impl From<Command> for PttEventType {
    fn from(value: Command) -> Self {
        match value {
            Command::Start => PttEventType::StartRecording,
            Command::Stop => PttEventType::StopRecording,
            Command::Toggle => PttEventType::ToggleRecording,
            Command::Partial => PttEventType::PartialTranscribe,
        }
    }
}

fn send_ptt_event(
    publisher: &Publisher<ipc::Service, PttEvent, ()>,
    event_type: PttEventType,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let event = PttEvent {
        event_type,
        timestamp,
    };

    let sample = publisher.loan_uninit()?;
    let sample = sample.write_payload(event);
    sample.send()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_default_env()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let node = NodeBuilder::new()
        .name(&"vypec".try_into()?)
        .create::<ipc::Service>()?;

    let ptt_service = node
        .service_builder(&"vype/ptt_events".try_into()?)
        .publish_subscribe::<PttEvent>()
        .max_publishers(8)
        .max_subscribers(4)
        .open_or_create()?;

    let publisher = ptt_service.publisher_builder().create()?;
    let event_type: PttEventType = args.command.into();
    send_ptt_event(&publisher, event_type)?;
    thread::sleep(Duration::from_millis(75));

    info!("Sent command: {:?}", args.command);
    Ok(())
}
